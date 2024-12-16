//! Lowering the exports for cross-context calls.

use std::collections::BTreeMap;

use miden_assembly::Spanned;
use midenc_hir::{
    diagnostics::Severity,
    pass::AnalysisManager,
    types::Abi::{self, Canonical},
    ComponentBuilder, ComponentExport, FunctionType, InstBuilder, InterfaceFunctionIdent,
};
use midenc_session::{DiagnosticsHandler, Session};

use super::flat::{
    assert_core_wasm_signature_equivalence, flatten_function_type, needs_transformation,
    FlatteningDirection,
};
use crate::{stage::Stage, CompilerResult, LinkerInput};

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

// TODO: NO! Reverse - rename these stages to be inline with WASM CM!!!
// swap `lift` and `lower` in the component import/export pretty-printing to sync with
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

        // dbg!(&component.exports());
        // dbg!(&component.modules().keys());

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

        // dbg!(&component.exports());
        // dbg!(&component.modules().keys());

        Ok(LinkerInput::Hir(component.into()))
    }
}

fn generate_lowering_function(
    component_builder: &mut ComponentBuilder,
    export_id: InterfaceFunctionIdent,
    export: ComponentExport,
    diagnostics: &DiagnosticsHandler,
) -> CompilerResult<ComponentExport> {
    let export_func_sig = component_builder
        .signature(&export.function)
        .ok_or({
            let message = format!(
                "Miden CCABI export lowering generation. Cannot find signature for exported \
                 function {}",
                export.function
            );
            diagnostics.diagnostic(Severity::Error).with_message(message).into_report()
        })?
        .clone();

    // get or create the module for the interface
    let module_id = export_id.interface.full_name;
    let mut module_builder = component_builder.module(module_id);
    let cross_ctx_export_sig =
        flatten_function_type(&export.function_ty, FlatteningDirection::Lift).map_err(|e| {
            let message = format!(
                "Miden CCABI export lowering generation. Signature for exported function {} \
                 requires flattening. Error: {}",
                export.function, e
            );
            diagnostics.diagnostic(Severity::Error).with_message(message).into_report()
        })?;
    if needs_transformation(&cross_ctx_export_sig) {
        let message = format!(
            "Miden CCABI export lowering generation. Signature for exported function {} requires \
             lowering. This is not yet supported",
            export.function
        );
        return Err(diagnostics.diagnostic(Severity::Error).with_message(message).into_report());
    }
    assert_core_wasm_signature_equivalence(&export_func_sig, &cross_ctx_export_sig);

    let mut builder = module_builder.function(export_id.function, cross_ctx_export_sig.clone())?;
    let entry = builder.current_block();
    let params = builder.block_params(entry).to_vec();

    let dfg = builder.data_flow_graph_mut();
    // import the Wasm core function
    if dfg.get_import(&export.function).is_none() {
        dfg.import_function(
            export.function.module,
            export.function.function,
            export_func_sig.clone(),
        )
        .map_err(|_e| {
            let message = format!(
                "Miden CCABI export lowering generation. Lowering function with name {} in module \
                 {} with signature {export_func_sig:?} is already imported (function call) with a \
                 different signature",
                export.function.function, export.function.module
            );
            diagnostics.diagnostic(Severity::Error).with_message(message).into_report()
        })?;
    }
    let span = export.function.function.span();
    let call = builder.ins().exec(export.function, &params, span);
    // dbg!(&sig);
    let result = builder.first_result(call);
    builder.ins().ret(Some(result), span);
    let function_id = builder.build()?;
    module_builder.build()?;
    let component_export = ComponentExport {
        function: function_id,
        function_ty: FunctionType {
            abi: Abi::Canonical,
            ..export.function_ty
        },
        ..export
    };
    Ok(component_export)
}
