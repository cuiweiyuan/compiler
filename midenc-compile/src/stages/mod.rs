use either::Either::{self, *};
use midenc_codegen_masm as masm;
use midenc_frontend_wasm as wasm;
use midenc_hir::{
    self as hir,
    parser::ast,
    pass::{AnalysisManager, ConversionPass, RewritePass},
};
use midenc_session::{
    diagnostics::{IntoDiagnostic, Report, WrapErr},
    OutputMode, Session,
};

use super::Stage;
use crate::{CompilerResult, CompilerStopped};

mod assemble;
mod codegen;
mod lift_cross_ctx;
mod link;
mod lower_cross_ctx;
mod parse;
mod rewrite;
mod sema;

pub use self::{
    assemble::{Artifact, AssembleStage},
    codegen::CodegenStage,
    lift_cross_ctx::LiftImportsCrossCtxStage,
    link::{LinkerInput, LinkerOutput, LinkerStage},
    lower_cross_ctx::LowerExportsCrossCtxStage,
    parse::{ParseOutput, ParseStage},
    rewrite::ApplyRewritesStage,
    sema::SemanticAnalysisStage,
};
