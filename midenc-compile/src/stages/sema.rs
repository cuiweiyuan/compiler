use midenc_hir::ComponentBuilder;

use super::*;

/// This stage of compilation takes the output of the parsing
/// stage and loads it into an HIR module for later stages.
///
/// This may involve additional validation/semantic analysis, hence the name.
pub struct SemanticAnalysisStage;
impl Stage for SemanticAnalysisStage {
    type Input = ParseOutput;
    type Output = LinkerInput;

    fn run(
        &mut self,
        input: Self::Input,
        analyses: &mut AnalysisManager,
        session: &Session,
    ) -> CompilerResult<Self::Output> {
        let parse_only = session.parse_only();
        let output = match input {
            ParseOutput::Ast(ast) if parse_only => {
                log::debug!("skipping semantic analysis (parse-only=true)");
                session.emit(OutputMode::Text, &ast).into_diagnostic()?;
                return Err(CompilerStopped.into());
            }
            ParseOutput::Ast(ast) => {
                log::debug!("performing semantic analysis on ast module '{}'", ast.name.as_str());
                session.emit(OutputMode::Text, &ast).into_diagnostic()?;
                let mut convert_to_hir = ast::ConvertAstToHir;
                let module = Box::new(convert_to_hir.convert(ast, analyses, session)?);
                let component =
                    ComponentBuilder::new(&session.diagnostics).with_module(module)?.build();
                LinkerInput::Hir(component.into())
            }
            ParseOutput::Hir(component) if parse_only => {
                log::debug!("skipping semantic analysis (parse-only=true)");
                session.emit(OutputMode::Text, &component).into_diagnostic()?;
                return Err(CompilerStopped.into());
            }
            ParseOutput::Hir(component) => {
                log::debug!(
                    "no semantic analysis required, '{}' is already valid hir Compinent",
                    component.name()
                );
                session.emit(OutputMode::Text, &component).into_diagnostic()?;
                LinkerInput::Hir(component)
            }
            ParseOutput::Masm(masm) if parse_only => {
                log::debug!("skipping semantic analysis (parse-only=true)");
                session.emit(OutputMode::Text, &masm).into_diagnostic()?;
                return Err(CompilerStopped.into());
            }
            ParseOutput::Masm(masm) => {
                log::debug!(
                    "no semantic analysis required, '{}' is already valid hir",
                    masm.id.as_str()
                );
                session.emit(OutputMode::Text, &masm).into_diagnostic()?;
                LinkerInput::Masm(masm)
            }
        };
        if session.analyze_only() {
            log::debug!("stopping compiler early (analyze-only=true)");
            Err(CompilerStopped.into())
        } else {
            Ok(output)
        }
    }
}
