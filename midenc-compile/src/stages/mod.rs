use midenc_codegen_masm as masm;
use midenc_frontend_wasm as wasm;
use midenc_hir::{
    self as hir,
    parser::ast,
    pass::{AnalysisManager, ConversionPass, RewritePass},
};
use midenc_session::Session;

use super::Stage;
use crate::CompilerResult;

mod codegen;
mod link;
mod parse;
mod rewrite;
mod sema;

pub use self::{
    codegen::CodegenStage,
    link::{LinkerInput, LinkerOutput, LinkerStage},
    parse::{ParseOutput, ParseStage},
    rewrite::ApplyRewritesStage,
    sema::SemanticAnalysisStage,
};
