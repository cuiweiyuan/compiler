use midenc_session::InputFile;
use std::path::Path;

use super::*;

/// This represents the output of the parser, depending on the type
/// of input that was parsed/loaded.
pub enum ParseOutput {
    /// We parsed HIR into the AST from text
    Ast(Box<ast::Module>),
    /// We parsed HIR from a Wasm module or other binary format
    Hir(Box<hir::Module>),
}

/// This stage of compilation is where we parse input files into the
/// earliest representation supported by the input file type. Later
/// stages will handle lowering as needed.
pub struct ParseStage;
impl Stage for ParseStage {
    type Input = InputFile;
    type Output = ParseOutput;

    fn run(
        &mut self,
        input: Self::Input,
        _analyses: &mut AnalysisManager,
        session: &Session,
    ) -> DriverResult<Self::Output> {
        use midenc_session::{FileType, InputType};
        // Track when compilation began
        let file_type = input.file_type();
        match &input.file {
            InputType::Real(ref path) => match file_type {
                FileType::Hir => self.parse_ast_from_file(path.as_ref(), &session),
                FileType::Wasm => self.parse_hir_from_wasm_file(path.as_ref(), &session),
                unsupported => unreachable!("unsupported file type: {unsupported}"),
            },
            InputType::Stdin { ref input, .. } => match file_type {
                FileType::Hir => self.parse_ast_from_bytes(&input, &session),
                FileType::Wasm => self.parse_hir_from_wasm_bytes(&input, &session),
                unsupported => unreachable!("unsupported file type: {unsupported}"),
            },
        }
    }
}
impl ParseStage {
    fn parse_ast_from_file(&self, path: &Path, session: &Session) -> DriverResult<ParseOutput> {
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::with_capacity(1024);
        file.read_to_end(&mut bytes)?;
        self.parse_ast_from_bytes(&bytes, session)
    }

    fn parse_ast_from_bytes(&self, bytes: &[u8], session: &Session) -> DriverResult<ParseOutput> {
        use miden_hir::parser::Parser;
        use std::io::{Error, ErrorKind};

        let source = core::str::from_utf8(bytes).map_err(|_| {
            DriverError::Io(Error::new(
                ErrorKind::InvalidInput,
                "input is not valid utf-8",
            ))
        })?;
        let parser = Parser::new(session);
        let ast = Box::new(parser.parse_str(source)?);
        session.emit(&ast)?;

        Ok(ParseOutput::Ast(ast))
    }

    fn parse_hir_from_wasm_file(
        &self,
        path: &Path,
        session: &Session,
    ) -> DriverResult<ParseOutput> {
        use std::io::Read;

        let mut file = std::fs::File::open(path)?;
        let mut bytes = Vec::with_capacity(1024);
        file.read_to_end(&mut bytes)?;
        self.parse_hir_from_wasm_bytes(&bytes, session)
    }

    fn parse_hir_from_wasm_bytes(
        &self,
        bytes: &[u8],
        session: &Session,
    ) -> DriverResult<ParseOutput> {
        let config = wasm::WasmTranslationConfig::default();
        let module = wasm::translate_module(bytes, &config, &session.diagnostics)?;

        Ok(ParseOutput::Hir(Box::new(module)))
    }
}