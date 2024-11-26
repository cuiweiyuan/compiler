//! Lowering the exports for cross-context calls.

use midenc_hir::pass::AnalysisManager;
use midenc_session::Session;

use super::LinkerInput;
use crate::{stage::Stage, CompilerResult};

/// Generates lowering for exports for the cross-context calls according to the Miden ABI.
///
/// For every exported function load the arguments from the stack or the advice provider and put
/// the result on the stack/advice provider.
///
/// After this stage all exported functons are expected to be called using the Miden ABI for
/// cross-context calls, i.e. using the stack and the advice provider for arguments and results.
pub struct LowerExportsCrossCtxStage;

// TODO: load the rodata into the memory in the lowering to ensure that the fresh context is
// correctly initialized
// TODO: Don't lower the note script's entry point

impl Stage for LowerExportsCrossCtxStage {
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
