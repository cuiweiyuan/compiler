//! Lifting the imports into the Miden ABI for the cross-context calls

use std::collections::{BTreeMap, VecDeque};

use midenc_hir::{
    diagnostics::Severity, pass::AnalysisManager, types::Abi, AbiParam, Block, Call, CallConv,
    ComponentBuilder, ComponentImport, Function, FunctionIdent, FunctionType, InstBuilder,
    Instruction, Linkage, MidenAbiImport, Signature, SourceSpan, Symbol, Type, UnsafeRef,
};
use midenc_session::{DiagnosticsHandler, Session};

use super::LinkerInput;
use crate::{stage::Stage, CompilerResult};

/// Generates lifting for imports for the cross-context calls according to the Miden ABI.
///
/// For each component import ensure a module for each interface that has a lifting function i.e. a
/// function that takes the arguments according to the Wasm CABI, lowers them to the cross-context
/// Miden ABI, calls the imported function with the lowered arguments, takes the result
/// (cross-context Miden ABI), and lifts it to the Wasm CABI.
/// The calls to the component import is switched to the generated lifting function.
pub struct LiftImportsCrossCtxStage;

impl Stage for LiftImportsCrossCtxStage {
    type Input = LinkerInput;
    type Output = LinkerInput;

    fn run(
        &mut self,
        input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        let component = if let LinkerInput::Hir(component) = input {
            component
        } else {
            return Ok(input);
        };

        // So far only hardcoded lift imports for the cross_ctx_note
        if component.name().as_str() != "cross_ctx_note" {
            return Ok(LinkerInput::Hir(component));
        }

        // dbg!(&component.imports());
        // dbg!(&component.modules().keys());

        let mut component_builder = ComponentBuilder::load(*component, &session.diagnostics);

        let mut lifted_imports: BTreeMap<FunctionIdent, ComponentImport> = BTreeMap::new();
        let imports = component_builder.imports().clone();
        for (id, import) in imports.into_iter() {
            if let ComponentImport::MidenAbiImport(_) = import {
                // skip imports that are already lifted
                lifted_imports.insert(id, import);
                continue;
            }
            let cabi_import = import.unwrap_canon_abi_import();
            let import_func_id = FunctionIdent {
                module: cabi_import.interface_function.interface.full_name.into(),
                function: cabi_import.interface_function.function.into(),
            };
            let (new_import, lifting_func_id) = generate_lifting_function(
                &mut component_builder,
                import_func_id,
                &session.diagnostics,
            )?;
            lifted_imports.insert(lifting_func_id, new_import.into());

            call_lifting_function(
                &mut component_builder,
                lifting_func_id,
                import_func_id,
                analyses,
                session,
            )?;
        }

        let component_builder = component_builder.with_imports(lifted_imports);

        let component = component_builder.build();

        // dbg!(&component.imports());
        // dbg!(&component.modules().keys());

        Ok(LinkerInput::Hir(component.into()))
    }
}

/// Generates the lifting function (cross-context Miden ABI -> Wasm CABI) for the given import function.
fn generate_lifting_function(
    component_builder: &mut ComponentBuilder<'_>,
    import_func_id: FunctionIdent,
    diagnostics: &DiagnosticsHandler,
) -> CompilerResult<(MidenAbiImport, FunctionIdent)> {
    // get or create the module for the interface
    let lifting_module_id =
        Symbol::intern(format!("lift-imports-{}", import_func_id.module.as_str()));
    dbg!(&lifting_module_id);
    // TODO: prefix the module name with "lift-imports-"? The same for the lowering exports module.
    let mut module_builder = component_builder.module(lifting_module_id);
    // TODO: put the core function signature (as imported in the core module) in the component import
    let import_core_sig = Signature {
        params: vec![AbiParam::new(Type::Felt)],
        results: vec![AbiParam::new(Type::Felt)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut builder = module_builder.function(import_func_id.function, import_core_sig.clone())?;
    let entry = builder.current_block();
    let params = builder.block_params(entry).to_vec();

    // TODO: analyze the signature and speculate what cross-context Miden ABI signature would
    // export have.
    // For now just assume passing <16 felts and returning 1 and copy the signature
    let import_lowered_sig = Signature {
        params: vec![AbiParam::new(Type::Felt)],
        results: vec![AbiParam::new(Type::Felt)],
        // TODO: add CallConv::CrossCtx
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let dfg = builder.data_flow_graph_mut();
    if dfg.get_import(&import_func_id).is_none() {
        dfg.import_function(
            import_func_id.module,
            import_func_id.function,
            import_lowered_sig.clone(),
        )
        .map_err(|_e| {
            let message = format!(
                "Lifting function with name {} in module {} with signature {import_lowered_sig:?} \
                 is already imported (function call) with a different signature",
                import_func_id.function, import_func_id.module
            );
            diagnostics.diagnostic(Severity::Error).with_message(message).into_report()
        })?;
    }
    // TODO: use the span from the caller
    // TODO: lower the params from the Wasm CABI to the cross-context Miden ABI
    let call = builder.ins().call(import_func_id, &params, SourceSpan::UNKNOWN);
    // dbg!(&sig);
    // TODO: lift the result from the cross-context Miden ABI to Wasm CABI
    let result = builder.first_result(call);
    builder.ins().ret(Some(result), SourceSpan::UNKNOWN);
    let function_id = builder.build()?;
    module_builder.build()?;
    let component_import = MidenAbiImport::new(FunctionType {
        abi: Abi::Canonical,
        params: import_lowered_sig.params.into_iter().map(|p| p.ty).collect(),
        results: import_lowered_sig.results.into_iter().map(|r| r.ty).collect(),
    });
    // dbg!(&component_import);
    // dbg!(&function_id);

    Ok((component_import, function_id))
}

/// Rewrites the calls to the function `from` to the function `to` in the given function.
/// Returns `true` if any call was rewritten.
fn rewrite_calls(function: &mut Function, from: FunctionIdent, to: FunctionIdent) -> bool {
    let mut dirty = false;
    let mut worklist = VecDeque::<Block>::default();
    for block in function.dfg.blocks.keys() {
        worklist.push_back(block);
    }

    for b in worklist {
        let block = &mut function.dfg.blocks[b];
        // Take the list of instructions away from the block to simplify traversing the block
        let mut insts = block.insts.take();
        // Take each instruction out of the list, top to bottom, modify it, then
        // add it back to the instruction list of the block directly. This ensures
        // we traverse the list and rewrite instructions in a single pass without
        // any additional overhead
        while let Some(inst) = insts.pop_front() {
            let mut inst = unsafe { UnsafeRef::into_box(inst) };
            let instruction: &mut Instruction = inst.as_mut();
            match instruction {
                Instruction::Call(Call { ref mut callee, .. }) => {
                    // Rewrite the call instruction
                    // rewrite_call(call, &mut function.dfg.value_lists, rewrites);
                    if callee == &from {
                        *callee = to;
                        dirty = true;
                    }
                }
                _op => (),
            }

            block.insts.push_back(UnsafeRef::from_box(inst));
        }
    }
    dirty
}

/// Replaces calls to the imported functions with calls to the lifting functions.
fn call_lifting_function(
    component_builder: &mut ComponentBuilder<'_>,
    lifting_func_id: FunctionIdent,
    import_func_id: FunctionIdent,
    analyses: &mut AnalysisManager,
    session: &Session,
) -> Result<(), miden_assembly::Report> {
    let mut modules = Vec::new();
    for (id, mut module) in component_builder.take_modules() {
        if module.name == lifting_func_id.module {
            // Skip the module with the lifting function
            modules.push((id, module));
            continue;
        }
        // Removing a function via this cursor will move the cursor to
        // the next function in the module. Once the end of the module
        // is reached, the cursor will point to the null object, and
        // `remove` will return `None`.
        let mut cursor = module.cursor_mut();
        // let mut dirty = false;
        while let Some(mut function) = cursor.remove() {
            // // Apply rewrite
            // if self.0.should_apply(&function, session) {
            //     dirty = true;
            //     self.0.apply(&mut function, analyses, session)?;
            //     analyses.invalidate::<crate::Function>(&function.id);
            // }

            // TODO: put the core function signature (as imported in the core module) in the component import
            // as in the generate_lifting_function
            let import_core_sig = Signature {
                params: vec![AbiParam::new(Type::Felt)],
                results: vec![AbiParam::new(Type::Felt)],
                cc: CallConv::SystemV,
                linkage: Linkage::External,
            };

            if rewrite_calls(&mut function, import_func_id, lifting_func_id) {
                // Invalidate the analyses for the function since we've modified it
                analyses.invalidate::<Function>(&function.id);
                // Import the lifting function if it's not already imported
                let dfg = &mut function.dfg;
                if dfg.get_import(&lifting_func_id).is_none() {
                    dfg.import_function(
                        lifting_func_id.module,
                        lifting_func_id.function,
                        import_core_sig.clone(),
                    )
                    .map_err(|_e| {
                        let message = format!(
                            "Lifting function with name {} in module {} with signature \
                             {import_core_sig:?} is already imported (function call) with a \
                             different signature",
                            import_func_id.function, import_func_id.module
                        );
                        session
                            .diagnostics
                            .diagnostic(Severity::Error)
                            .with_message(message)
                            .into_report()
                    })?;
                }
            }

            // Add the function back to the module
            //
            // We add it before the current position of the cursor
            // to ensure that we don't interfere with our traversal
            // of the module top to bottom
            cursor.insert_before(function);
        }
        modules.push((id, module));
    }
    component_builder.set_modules(modules.into_iter().collect());
    Ok(())
}
