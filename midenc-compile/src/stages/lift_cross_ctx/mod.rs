//! Lifting the imports into the Miden ABI for the cross-context calls

use midenc_hir::pass::AnalysisManager;
use midenc_session::Session;

use super::LinkerInput;
use crate::{stage::Stage, CompilerResult};

/// Generates lifting for imports for the cross-context calls according to the Miden ABI.
///
/// For every imported function call put the arguments on the stack or the advice provider and load
/// the result from the stack/advice provider into the memory according to the pointer after the
/// call.
pub struct LiftImportsCrossCtxStage;

impl Stage for LiftImportsCrossCtxStage {
    type Input = LinkerInput;
    type Output = LinkerInput;

    fn run(
        &mut self,
        input: Self::Input,
        _analyses: &mut AnalysisManager,
        _session: &Session,
    ) -> CompilerResult<Self::Output> {
        Ok(input)
    }
}
