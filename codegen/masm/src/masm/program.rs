use std::{fmt, path::Path, sync::Arc};

use hir::{Signature, Symbol};
use miden_assembly::{
    ast::{ModuleKind, ProcedureName},
    LibraryNamespace,
};
use miden_core::crypto::hash::Rpo256;
use midenc_hir::{
    self as hir, diagnostics::Report, DataSegmentTable, Felt, FieldElement, FunctionIdent, Ident,
    SourceSpan,
};
use midenc_hir_analysis::GlobalVariableAnalysis;
use midenc_session::Session;

use super::{module::Modules, *};

inventory::submit! {
    midenc_session::CompileFlag::new("test_harness")
        .long("test-harness")
        .action(midenc_session::FlagAction::SetTrue)
        .help("If present, causes the code generator to emit extra code for the VM test harness")
        .help_heading("Testing")
}

/// A [Program] represents a complete set of modules which are intended to
/// be shipped together as an artifact, either as an executable, or as a library
/// to be integrated into a larger executable.
///
/// Modules are stored in a [Program] in a b-tree map, keyed by the module name.
/// This is done to make accessing modules by name efficient, and to ensure a
/// stable ordering for compiled programs when emitted as text.
#[derive(Default)]
pub struct Program {
    /// The set of modules which belong to this program
    modules: Modules,
    /// The data segment table for this program
    pub segments: DataSegmentTable,
    /// The function identifier for the program entrypoint, if applicable
    pub entrypoint: Option<FunctionIdent>,
    /// The address of the `__stack_pointer` global, if such a global has been defined
    stack_pointer: Option<u32>,
}
impl Program {
    /// Create a new, empty [Program]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Create a new [Program] initialized from a [DataSegmentTable], a set of [Module]s, and an
    /// optional entrypoint function.
    ///
    /// If the entrypoint is specified, a `main.masm` module will be generated which invokes that
    /// entrypoint on startup, after initializing the global heap of the root context, based on the
    /// provided data segment table.
    ///
    /// You should generally prefer to use [Program::from_hir], but this constructor allows you to
    /// manually produce a MASM program from its constituent parts.
    pub fn new<M>(entrypoint: Option<FunctionIdent>, segments: DataSegmentTable, modules: M) -> Self
    where
        M: IntoIterator<Item = Box<Module>>,
    {
        let mut module_tree = ModuleTree::default();
        for module in modules {
            module_tree.insert(module);
        }
        let modules = Modules::Open(module_tree);
        Self {
            modules,
            segments,
            entrypoint,
            stack_pointer: None,
        }
    }

    /// Create a new [Program] initialized from an [hir::Program].
    ///
    /// The resulting [Program] will have the following:
    ///
    /// * Data segments described by the original [hir::Program]
    /// * The entrypoint function which will be invoked after the initialization phase of startup
    /// * If an entrypoint is set, an executable [Module] which performs initialization and then
    ///   invokes the entrypoint
    ///
    /// None of the HIR modules will have been added yet
    pub fn from_hir(
        program: &hir::Program,
        globals: &GlobalVariableAnalysis<hir::Program>,
    ) -> Self {
        let stack_pointer = program.globals().find("__stack_pointer".parse().unwrap());
        let stack_pointer = if let Some(stack_pointer) = stack_pointer {
            let global_table_offset = globals.layout().global_table_offset();
            Some(global_table_offset + unsafe { program.globals().offset_of(stack_pointer) })
        } else {
            None
        };
        Self {
            modules: Modules::default(),
            segments: program.segments().clone(),
            entrypoint: program.entrypoint(),
            stack_pointer,
        }
    }

    /// Generate an executable module which when run expects the raw data segment data to be
    /// provided on the advice stack in the same order as initialization, and the operands of
    /// the entrypoint function on the operand stack.
    fn generate_main(&self, entrypoint: FunctionIdent, emit_test_harness: bool) -> Box<Module> {
        let mut exe = Box::new(Module::new(LibraryNamespace::Exec.into(), ModuleKind::Executable));
        let start_id = FunctionIdent {
            module: Ident::with_empty_span(Symbol::intern(LibraryNamespace::EXEC_PATH)),
            function: Ident::with_empty_span(Symbol::intern(ProcedureName::MAIN_PROC_NAME)),
        };
        let start_sig = Signature::new([], []);
        let mut start = Box::new(Function::new(start_id, start_sig));
        {
            let body = start.body_mut();
            self.emit_data_segment_initialization(body);
            if emit_test_harness {
                self.emit_test_harness(body);
            }
            body.push(Op::Exec(entrypoint), SourceSpan::default());
        }
        exe.push_back(start);
        exe
    }

    /// Generate an executable module which when run expects the raw data segment data to be
    /// provided on the advice stack in the same order as initialization, and the hash of a
    /// procedure in the set of modules of the program that you wish to invoke, followed by any
    /// operands to be passed as arguments.
    fn generate_dyn_main(&self, emit_test_harness: bool) -> Box<Module> {
        let mut exe = Box::new(Module::new(LibraryNamespace::Exec.into(), ModuleKind::Executable));
        let start_id = FunctionIdent {
            module: Ident::with_empty_span(Symbol::intern(LibraryNamespace::EXEC_PATH)),
            function: Ident::with_empty_span(Symbol::intern(ProcedureName::MAIN_PROC_NAME)),
        };
        let start_sig = Signature::new([], []);
        let mut start = Box::new(Function::new(start_id, start_sig));

        {
            let body = start.body_mut();
            self.emit_data_segment_initialization(body);
            if emit_test_harness {
                self.emit_test_harness(body);
            }
            body.push(Op::DynExec, SourceSpan::default());
        }

        exe.push_back(start);
        exe
    }

    fn emit_test_harness(&self, block: &mut Block) {
        let span = SourceSpan::default();

        // Advice Stack: [dest_ptr, num_words, ...]
        block.push(Op::AdvPush(2), span); // => [num_words, dest_ptr] on operand stack
        block.push(Op::Exec("std::mem::pipe_words_to_memory".parse().unwrap()), span);
        // Drop the commitment
        block.push(Op::Drop, span);
        // If we know the stack pointer address, update it to the value of `'write_ptr`, but cast
        // into the Rust address space (multiplying it by 16). So a word address of 1, is equal to
        // a byte address of 16, because each field element holds 4 bytes, and there are 4 elements
        // in a word.
        //
        // If we don't know the stack pointer, just drop the `'write_ptr` value
        if let Some(sp) = self.stack_pointer {
            block.push(Op::U32OverflowingMulImm(16), span);
            block.push(Op::Assertz, span);
            // Align the stack pointer to a word boundary
            let elem_addr = (sp / 4) + (sp % 4 > 0) as u32;
            let word_addr = (elem_addr / 4) + (elem_addr % 4 > 0) as u32;
            block.push(Op::MemStoreImm(word_addr), span);
        } else {
            block.push(Op::Drop, span);
        }
    }

    /// Emit the sequence of instructions necessary to consume rodata from the advice stack and
    /// populate the global heap with the data segments of this program, verifying that the
    /// commitments match.
    fn emit_data_segment_initialization(&self, block: &mut Block) {
        // Emit data segment initialization code
        //
        // NOTE: This depends on the program being executed with the data for all data
        // segments having been pushed on the advice stack in the same order as visited
        // here, with the same encoding. The program will fail to execute if it is not
        // set up correctly.
        //
        // TODO(pauls): To facilitate automation of this, we should emit a file to disk
        // that includes the raw encoding of the data we expect to be placed on the advice
        // stack, in a manner which allows us to simply read that file as an array of felt
        // and use that directly via `AdviceInputs`
        let pipe_preimage_to_memory = "std::mem::pipe_preimage_to_memory".parse().unwrap();
        for segment in self.segments.iter() {
            // Don't bother emitting anything for zeroed segments
            if segment.is_zeroed() {
                continue;
            }
            let size = segment.size();
            let offset = segment.offset();
            let base = NativePtr::from_ptr(offset);
            let segment_data = segment.init();

            // TODO(pauls): Do we ever have a need for data segments which are not aligned
            // to an word boundary? If so, we need to implement that
            // support when emitting the entry for a program
            assert_eq!(
                base.offset,
                0,
                "unsupported data segment alignment {}: must be aligned to a 32 byte boundary",
                base.alignment()
            );
            assert_eq!(
                base.index,
                0,
                "unsupported data segment alignment {}: must be aligned to a 32 byte boundary",
                base.alignment()
            );

            // Compute the commitment for the data
            let num_elements = size.next_multiple_of(4) / 4;
            let num_words = num_elements.next_multiple_of(4) / 4;
            let mut elements = Vec::with_capacity(num_elements as usize);
            // TODO(pauls): If the word containing the first element overlaps with the
            // previous segment, then ensure the overlapping elements
            // are mixed together, so that the data is preserved, and
            // the commitment is correct
            let mut iter = segment_data.as_slice().iter().copied().array_chunks::<4>();
            elements.extend(iter.by_ref().map(|bytes| Felt::new(u32::from_be_bytes(bytes) as u64)));
            if let Some(remainder) = iter.into_remainder() {
                let mut chunk = [0u8; 4];
                for (i, byte) in remainder.into_iter().enumerate() {
                    chunk[i] = byte;
                }
                elements.push(Felt::new(u32::from_be_bytes(chunk) as u64));
            }
            elements.resize(num_elements as usize, Felt::ZERO);
            let digest = Rpo256::hash_elements(&elements);
            let span = SourceSpan::default();

            // COM
            block.push(Op::Pushw(digest.into()), span);
            // write_ptr
            block.push(Op::PushU32(base.waddr), span);
            // num_words
            block.push(Op::PushU32(num_words), span);
            // [num_words, write_ptr, COM, ..] -> [write_ptr']
            block.push(Op::Exec(pipe_preimage_to_memory), span);
            // drop write_ptr'
            block.push(Op::Drop, span);
        }
    }

    /// Get the expected [miden_processor::AdviceInputs] needed to execute this program.
    pub fn advice_inputs(&self) -> miden_processor::AdviceInputs {
        use miden_processor::AdviceInputs;

        let mut stack = Vec::with_capacity(
            self.segments.iter().map(|segment| segment.size() as usize).sum::<usize>() / 4,
        );

        let mut current_size = 0usize;
        for segment in self.segments.iter() {
            if segment.is_zeroed() {
                continue;
            }
            let size = segment.size() as usize;
            let num_elements = size.next_multiple_of(4) / 4;
            let num_words = num_elements.next_multiple_of(4) / 4;
            let mut iter = segment.init().as_slice().iter().copied().array_chunks::<4>();
            stack.extend(iter.by_ref().map(|bytes| Felt::new(u32::from_be_bytes(bytes) as u64)));
            if let Some(remainder) = iter.into_remainder() {
                let mut chunk = [0u8; 4];
                for (i, byte) in remainder.into_iter().enumerate() {
                    chunk[i] = byte;
                }
                stack.push(Felt::new(u32::from_be_bytes(chunk) as u64));
            }
            let num_elements_with_padding = num_words * 4;
            stack.resize(current_size + num_elements_with_padding, Felt::ZERO);
            current_size += num_elements_with_padding;
        }

        AdviceInputs::default().with_stack(stack)
    }

    /// Freezes this program, preventing further modifications
    pub fn freeze(mut self: Box<Self>) -> Arc<Program> {
        self.modules.freeze();
        Arc::from(self)
    }

    /// Get an iterator over the modules in this program
    pub fn modules(&self) -> impl Iterator<Item = &Module> + '_ {
        self.modules.iter()
    }

    /// Access the frozen module tree of this program, and panic if not frozen
    pub fn unwrap_frozen_modules(&self) -> &FrozenModuleTree {
        match self.modules {
            Modules::Frozen(ref modules) => modules,
            Modules::Open(_) => panic!("expected program to be frozen"),
        }
    }

    /// Insert a module into this program.
    ///
    /// The insertion order is not preserved - modules are ordered by name.
    ///
    /// NOTE: This function will panic if the program has been frozen
    pub fn insert(&mut self, module: Box<Module>) {
        self.modules.insert(module);
    }

    pub fn is_executable(&self) -> bool {
        self.entrypoint.is_some()
    }

    pub fn is_library(&self) -> bool {
        self.entrypoint.is_none()
    }

    /// Get a reference to a module in this program by name
    pub fn get<Q>(&self, name: &Q) -> Option<&Module>
    where
        Q: ?Sized + Ord,
        Ident: core::borrow::Borrow<Q>,
    {
        self.modules.get(name)
    }

    /// Returns true if this program contains a [Module] named `name`
    pub fn contains<N>(&self, name: N) -> bool
    where
        Ident: PartialEq<N>,
    {
        self.modules.iter().any(|m| m.id == name)
    }

    /// Write this [Program] to the given output directory.
    ///
    /// The provided [miden_diagnostics::CodeMap] is used for computing source locations.
    pub fn write_to_directory<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let path = path.as_ref();
        assert!(path.is_dir());

        for module in self.modules.iter() {
            module.write_to_directory(path)?;
        }

        if let Some(entry) = self.entrypoint {
            let main = self.generate_main(entry, /* test_harness= */ false);
            main.write_to_directory(path)?;
        }

        Ok(())
    }

    // Assemble this program to MAST
    pub fn assemble(&self, session: &Session) -> Result<Arc<miden_core::Program>, Report> {
        use miden_assembly::{Assembler, CompileOptions};
        use miden_stdlib::StdLibrary;

        let mut assembler = Assembler::new(session.source_manager.clone())
            .with_debug_mode(session.options.emit_debug_decorators())
            .with_library(&StdLibrary::default())?;
        for module in self.modules.iter() {
            let kind = module.kind;
            let module = module.to_ast().map(Box::new)?;
            assembler.add_module_with_options(
                module,
                CompileOptions {
                    kind,
                    warnings_as_errors: false,
                    path: None,
                },
            )?;
        }

        let emit_test_harness = session.get_flag("test_harness");
        let main = if let Some(entry) = self.entrypoint {
            self.generate_main(entry, emit_test_harness)
        } else {
            // Assemble a [Program] that will dyncall the procedure whose hash is placed on top of
            // the operand stack
            self.generate_dyn_main(emit_test_harness)
        };
        let main = main.to_ast().map(Box::new)?;
        println!("{main}");
        assembler.assemble_program(main).map(Arc::new)
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for module in self.modules.iter() {
            // Don't print intrinsic modules
            if module.id.as_str().starts_with("intrinsics::") {
                continue;
            }

            writeln!(f, "# mod {}\n", &module.name)?;
            writeln!(f, "{}", module)?;
        }

        Ok(())
    }
}
