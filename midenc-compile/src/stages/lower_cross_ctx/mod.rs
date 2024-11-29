//! Lowering the exports for cross-context calls.

use std::collections::BTreeMap;

use midenc_hir::{
    diagnostics::Severity, pass::AnalysisManager, types::Abi::Canonical, AbiParam, CallConv,
    ComponentBuilder, ComponentExport, FunctionType, InstBuilder, InterfaceFunctionIdent, Linkage,
    Signature, SourceSpan, Type,
};
use midenc_session::{DiagnosticsHandler, Session};

use super::LinkerInput;
use crate::{stage::Stage, CompilerResult};

/// Generates lowering for exports for the cross-context calls according to the Miden ABI.
///
/// For each component export ensure a module for each interface that has a lowering
/// function, i.e. a function that takes parameters according to the cross-context Miden
/// ABI, lifts them to the Wasm CABI, calls the core Wasm module exported function,
/// lowers the results to the cross-context Miden ABI
///
/// After this stage all exported functons are expected to be called using the Miden ABI for
/// cross-context calls, i.e. using the stack and the advice provider for arguments and results.
pub struct LowerExportsCrossCtxStage;

// TODO: load the rodata into the memory in the lowering to ensure that the fresh context is
// correctly initialized

// TODO: swap `lift` and `lower` in the component import/export pretty-printing to sync with
// this stage's terminology (an export is lowered, an import is lifted)

impl Stage for LowerExportsCrossCtxStage {
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

        let mut component_builder = ComponentBuilder::load(*component, &session.diagnostics);

        let mut lowered_exports: BTreeMap<InterfaceFunctionIdent, ComponentExport> =
            BTreeMap::new();
        let exports = component_builder.exports().clone();
        for (id, export) in exports.into_iter() {
            if let Canonical = export.function_ty.abi() {
                // skip exports that are already lowered
                lowered_exports.insert(id, export);
                continue;
            }
            if let Some(entrypoint) = &session.options.entrypoint {
                // skip the entrypoint
                let export_core_func = format!(
                    "{}::{}",
                    export.function.module.as_str(),
                    export.function.function.as_str()
                );
                if entrypoint == &export_core_func {
                    lowered_exports.insert(id, export);
                    continue;
                }
            }
            let new_export = generate_lowering_function(
                &mut component_builder,
                id,
                export,
                &session.diagnostics,
            )?;
            lowered_exports.insert(id, new_export);
        }

        let component_builder = component_builder.with_exports(lowered_exports);

        let component = component_builder.build();
        Ok(LinkerInput::Hir(component.into()))
    }
}

fn generate_lowering_function(
    component_builder: &mut ComponentBuilder,
    export_id: InterfaceFunctionIdent,
    export: ComponentExport,
    diagnostics: &DiagnosticsHandler,
) -> CompilerResult<ComponentExport> {
    // So far we only hardcoded the lowering for the process-felt function
    if export_id.interface.to_string() != "miden:cross-ctx-account/foo@1.0.0"
        && export_id.function.as_str() != "process-felt"
    {
        return Ok(export);
    }
    // get or create the module for the interface
    let module_id = export_id.interface.full_name;
    let mut module_builder = component_builder.module(module_id);
    // TODO: analyze the signature and speculate what cross-context Miden ABI signature we need to export.
    // For now just assume passing <16 felts and returning 1 and copy the signature
    let cc_export_sig = Signature {
        params: vec![AbiParam::new(Type::Felt)],
        results: vec![AbiParam::new(Type::Felt)],
        // TODO: add CallConv::CrossCtx
        cc: CallConv::SystemV,
        linkage: Linkage::External,
    };
    let mut builder = module_builder.function(export_id.function, cc_export_sig.clone())?;
    let entry = builder.current_block();
    let params = builder.block_params(entry).to_vec();
    // TODO: lift the params from the cross-context Miden ABI to the Wasm CABI

    // TODO: put the core function signature in the export
    let core_sig = Signature {
        params: vec![AbiParam::new(Type::Felt)],
        results: vec![AbiParam::new(Type::Felt)],
        cc: CallConv::SystemV,
        linkage: Linkage::Internal,
    };
    let dfg = builder.data_flow_graph_mut();
    // import the Wasm core function
    if dfg.get_import(&export.function).is_none() {
        dfg.import_function(export.function.module, export.function.function, core_sig)
            .map_err(|_e| {
                let message = format!(
                    "Function(callee of the lowering) with name {} in module {} with signature \
                     {cc_export_sig:?} is already imported (function call) with a different \
                     signature",
                    export.function.function, export.function.module
                );
                diagnostics.diagnostic(Severity::Error).with_message(message).into_report()
            })?;
    }
    // TODO: use the span from the callee
    let call = builder.ins().exec(export.function, &params, SourceSpan::UNKNOWN);
    // dbg!(&sig);
    // TODO: lower the result from the Wasm CABI to the cross-context Miden ABI
    let result = builder.first_result(call);
    builder.ins().ret(Some(result), SourceSpan::UNKNOWN);
    let function_id = builder.build()?;
    module_builder.build()?;
    let component_export = ComponentExport {
        function: function_id,
        function_ty: FunctionType {
            abi: Canonical,
            ..export.function_ty
        },
        ..export
    };
    Ok(component_export)
}
