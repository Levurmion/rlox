use crate::{
    lexer::{Lexer, LexerError},
    parser::{
        ast::{AstNode, ParserError},
        parser::Parser,
    },
};

#[derive(Debug)]
pub enum CompilerError {
    Lexer(LexerError),
    Parser(ParserError),
}

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&self, input: String) -> Result<Box<AstNode>, CompilerError> {
        let mut lexer = Lexer::new(input);
        match lexer.tokenize() {
            Err(err) => return Err(CompilerError::Lexer(err)),
            Ok(_) => {}
        }

        let mut parser = Parser::new(&lexer.tokens);
        match parser.parse() {
            Err(e) => {
                return Err(CompilerError::Parser(e));
            }
            Ok(ast) => Ok(ast),
        }
    }
}
