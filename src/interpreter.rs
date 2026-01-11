use std::{cell::Ref, result};

use crate::{
    lexer::{Lexer, LexerError, Token},
    parser::{
        ast::{AstNode, ParserError},
        parser::Parser,
    },
    repl::{Evaluator, EvaluatorOk},
};

#[derive(Debug)]
pub enum InterpreterError {
    Lexer(LexerError),
    ParsingError(String),
}

pub struct Interpreter {
    lexer: Lexer,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            lexer: Lexer::new(String::from("")),
        }
    }

    pub fn interpret(&mut self, input: String) -> Result<String, InterpreterError> {
        self.lexer = Lexer::new(input);
        match self.lexer.tokenize() {
            Err(lexer_err) => return Err(InterpreterError::Lexer(lexer_err)),
            _ => ..,
        };
        let mut parser = Parser::new(&self.lexer.tokens);

        match parser.parse() {
            Err(num) => Err(InterpreterError::ParsingError(format!(
                "Found {:?} errors: {:?}",
                num,
                parser
                    .errors
                    .iter()
                    .map(|e| {
                        match &*e.borrow() {
                            AstNode::ErrorNode { error, token } => (error.clone(), token.clone()),
                            _ => panic!("Expected error nodes only."),
                        }
                    })
                    .collect::<Vec<(ParserError, Option<Token>)>>()
            ))),
            Ok(_) => Ok(format!("{:#?}", parser.ast)),
        }
    }
}

impl Evaluator for Interpreter {
    fn eval(&mut self, input: String) -> Result<EvaluatorOk, String> {
        let interpret_result = self.interpret(input);
        match interpret_result {
            Ok(result) => Ok(EvaluatorOk::Clear(result)),
            Err(err) => Err(format!("{:#?}", err)),
        }
    }
}
