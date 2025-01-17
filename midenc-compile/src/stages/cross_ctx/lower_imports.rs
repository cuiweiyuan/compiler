//! lowering the imports into the Miden ABI for the cross-context calls

use std::collections::{BTreeMap, VecDeque};

use miden_assembly::Spanned;
use midenc_hir::{
    diagnostics::Severity, pass::AnalysisManager, types::Abi, Block, Call, CallConv,
    ComponentBuilder, ComponentImport, Function, FunctionIdent, FunctionType, InstBuilder,
    Instruction, MidenAbiImport, Signature, SourceSpan, Symbol, UnsafeRef,
};
use midenc_session::{DiagnosticsHandler, Session};

use super::flat::{
    assert_core_wasm_signature_equivalence, flatten_function_type, needs_transformation,
};
use crate::{stage::Stage, CompilerResult, LinkerInput};

/// Generates lowering for imports for the cross-context calls according to the Miden ABI.
///
/// For each component import ensure a module for each interface that has a lowering function i.e. a
/// function that takes the arguments according to the Wasm CABI, converts them to the cross-context
/// Miden ABI, calls the imported function with the lowered arguments, takes the result
/// (cross-context Miden ABI), and converts it to the Wasm CABI.
/// The calls to the component import is switched to the generated lowering function.
pub struct LowerImportsCrossCtxStage;

impl Stage for LowerImportsCrossCtxStage {
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

        // dbg!(&component.imports());
        // dbg!(&component.modules().keys());

        let mut component_builder = ComponentBuilder::load(*component, &session.diagnostics);

        let mut lowered_imports: BTreeMap<FunctionIdent, ComponentImport> = BTreeMap::new();
        let imports = component_builder.imports().clone();
        for (core_import_func_id, import) in imports.into_iter() {
            if let ComponentImport::MidenAbiImport(_) = import {
                // skip imports that are already lowered
                lowered_imports.insert(core_import_func_id, import);
                continue;
            }
            let cabi_import = import.unwrap_canon_abi_import();
            let import_func_id = FunctionIdent {
                module: cabi_import.interface_function.interface.full_name.into(),
                function: cabi_import.interface_function.function.into(),
            };

            let import_func_sig = component_builder
                .import_signature(&import_func_id)
                .ok_or({
                    let message = format!(
                        "Miden CCABI import lowering generation. Cannot find signature for Wasm \
                         CABI imported function {}",
                        import_func_id
                    );
                    session
                        .diagnostics
                        .diagnostic(Severity::Error)
                        .with_message(message)
                        .into_report()
                })?
                .clone();

            let (new_import, lowering_func_id) = generate_lowering_function(
                &mut component_builder,
                &cabi_import.interface_function_ty,
                import_func_id,
                import_func_sig.clone(),
                core_import_func_id.function.span(),
                &session.diagnostics,
            )?;
            lowered_imports.insert(lowering_func_id, new_import.into());

            call_lowering_function(
                &mut component_builder,
                lowering_func_id,
                import_func_id,
                import_func_sig,
                analyses,
                session,
            )?;
        }

        let component_builder = component_builder.with_imports(lowered_imports);

        let component = component_builder.build();

        // dbg!(&component.imports());
        // dbg!(&component.modules().keys());

        Ok(LinkerInput::Hir(component.into()))
    }
}

/// Generates the lowering function (cross-context Miden ABI -> Wasm CABI) for the given import function.
fn generate_lowering_function(
    component_builder: &mut ComponentBuilder<'_>,
    high_func_ty: &FunctionType,
    import_func_id: FunctionIdent,
    import_func_sig: Signature,
    span: SourceSpan,
    diagnostics: &DiagnosticsHandler,
) -> CompilerResult<(MidenAbiImport, FunctionIdent)> {
    // get or create the module for the interface
    let lowering_module_id =
        Symbol::intern(format!("lower-imports-{}", import_func_id.module.as_str()));
    // dbg!(&lowering_module_id);
    let mut module_builder = component_builder.module(lowering_module_id);

    let import_lowered_sig =
        flatten_function_type(high_func_ty, CallConv::CanonLower).map_err(|e| {
            let message = format!(
                "Miden CCABI import lowering generation. Signature for imported function {} \
                 requires flattening. Error: {}",
                import_func_id.function, e
            );
            diagnostics.diagnostic(Severity::Error).with_message(message).into_report()
        })?;

    if needs_transformation(&import_lowered_sig) {
        let message = format!(
            "Miden CCABI export lowering generation. Signature for exported function {} requires \
             lowering. This is not supported yet.",
            import_func_id
        );
        return Err(diagnostics.diagnostic(Severity::Error).with_message(message).into_report());
    }
    assert_core_wasm_signature_equivalence(&import_func_sig, &import_lowered_sig);

    let mut builder = module_builder.function(import_func_id.function, import_func_sig)?;
    let entry = builder.current_block();
    let params = builder.block_params(entry).to_vec();

    let dfg = builder.data_flow_graph_mut();
    if dfg.get_import(&import_func_id).is_none() {
        dfg.import_function(
            import_func_id.module,
            import_func_id.function,
            import_lowered_sig.clone(),
        )
        .map_err(|_e| {
            let message = format!(
                "lowering function with name {} in module {} with signature \
                 {import_lowered_sig:?} is already imported (function call) with a different \
                 signature",
                import_func_id.function, import_func_id.module
            );
            diagnostics.diagnostic(Severity::Error).with_message(message).into_report()
        })?;
    }
    // NOTE: handle lifting/lowering for non-scalar types
    // see https://github.com/0xPolygonMiden/compiler/pull/357
    let call = builder.ins().call(import_func_id, &params, span);
    // dbg!(&sig);
    let result = builder.first_result(call);
    builder.ins().ret(Some(result), span);
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

/// Replaces calls to the imported functions with calls to the lowering functions.
fn call_lowering_function(
    component_builder: &mut ComponentBuilder<'_>,
    lowering_func_id: FunctionIdent,
    import_func_id: FunctionIdent,
    import_func_sig: Signature,
    analyses: &mut AnalysisManager,
    session: &Session,
) -> Result<(), miden_assembly::Report> {
    let mut modules = Vec::new();
    for (id, mut module) in component_builder.take_modules() {
        if module.name == lowering_func_id.module {
            // Skip the module with the lowering function
            modules.push((id, module));
            continue;
        }
        // Removing a function via this cursor will move the cursor to
        // the next function in the module. Once the end of the module
        // is reached, the cursor will point to the null object, and
        // `remove` will return `None`.
        let mut cursor = module.cursor_mut();
        while let Some(mut function) = cursor.remove() {
            if rewrite_calls(&mut function, import_func_id, lowering_func_id) {
                // Invalidate the analyses for the function since we've modified it
                analyses.invalidate::<Function>(&function.id);
                // Import the lowering function if it's not already imported
                let dfg = &mut function.dfg;
                if dfg.get_import(&lowering_func_id).is_none() {
                    dfg.import_function(
                        lowering_func_id.module,
                        lowering_func_id.function,
                        import_func_sig.clone(),
                    )
                    .map_err(|_e| {
                        let message = format!(
                            "lowering function with name {} in module {} with signature \
                             {import_func_sig:?} is already imported (function call) with a \
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
