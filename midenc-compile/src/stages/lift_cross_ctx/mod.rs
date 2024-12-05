//! Lifting the imports into the Miden ABI for the cross-context calls

use std::collections::BTreeMap;

use midenc_hir::{
    diagnostics::Severity, pass::AnalysisManager, types::Abi, AbiParam, CallConv, CanonAbiImport,
    ComponentBuilder, ComponentImport, FunctionIdent, FunctionType, InstBuilder, Linkage,
    MidenAbiImport, Signature, SourceSpan, Symbol, Type,
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
        _analyses: &mut AnalysisManager,
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
            let (new_import, lifting_func_id) = generate_lifting_function(
                &mut component_builder,
                id,
                import.unwrap_canon_abi_import(),
                &session.diagnostics,
            )?;
            lifted_imports.insert(lifting_func_id, new_import.into());

            // TODO: find all the calls to the component import and replace them with the generated lifting function
        }

        let component_builder = component_builder.with_imports(lifted_imports);

        let component = component_builder.build();

        // dbg!(&component.imports());
        // dbg!(&component.modules().keys());

        Ok(LinkerInput::Hir(component.into()))
    }
}

fn generate_lifting_function(
    component_builder: &mut ComponentBuilder<'_>,
    _core_import_function: FunctionIdent,
    import: &CanonAbiImport,
    diagnostics: &DiagnosticsHandler,
) -> CompilerResult<(MidenAbiImport, FunctionIdent)> {
    // get or create the module for the interface
    let module_id = Symbol::intern(format!(
        "lift-imports-{}",
        import.interface_function.interface.full_name.as_str()
    ));
    dbg!(&module_id);
    // TODO: prefix the module name with "lift-imports-"? The same for the lowering exports module.
    let mut module_builder = component_builder.module(module_id);
    // TODO: put the core function signature (as imported in the core module) in the component import
    let import_core_sig = Signature {
        params: vec![AbiParam::new(Type::Felt)],
        results: vec![AbiParam::new(Type::Felt)],
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut builder =
        module_builder.function(import.interface_function.function, import_core_sig.clone())?;
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
    // import the Wasm CM interface function
    let import_func_id = FunctionIdent {
        module: import.interface_function.interface.full_name.into(),
        function: import.interface_function.function.into(),
    };
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
