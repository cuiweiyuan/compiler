//! Lifting the imports into the Miden ABI for the cross-context calls

use std::collections::BTreeMap;

use midenc_hir::{pass::AnalysisManager, ComponentBuilder, ComponentImport, FunctionIdent};
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

        let mut component_builder = ComponentBuilder::load(*component, &session.diagnostics);

        let mut lifted_imports: BTreeMap<FunctionIdent, ComponentImport> = BTreeMap::new();
        let imports = component_builder.imports().clone();
        for (id, import) in imports.into_iter() {
            if let ComponentImport::MidenAbiImport(_) = import {
                // skip imports that are already lifted
                lifted_imports.insert(id, import);
                continue;
            }
            let new_import = generate_lifting_function(
                &mut component_builder,
                id,
                import,
                &session.diagnostics,
            )?;
            lifted_imports.insert(id, new_import);

            // TODO: find all the calls to the component import and replace them with the generated lifting function
        }

        let component_builder = component_builder.with_imports(lifted_imports);

        let component = component_builder.build();
        // dbg!(&component.imports());
        // dbg!(&component.modules().len());
        Ok(LinkerInput::Hir(component.into()))
    }
}

fn generate_lifting_function(
    _component_builder: &mut ComponentBuilder<'_>,
    _id: FunctionIdent,
    import: ComponentImport,
    _diagnostics: &DiagnosticsHandler,
) -> CompilerResult<ComponentImport> {
    // TODO: implement the lifting function generation
    Ok(import)
}
