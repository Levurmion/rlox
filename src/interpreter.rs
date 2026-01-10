use std::result;

use crate::{
    lexer::{Lexer, LexerError},
    repl::{Evaluator, EvaluatorOk},
};

#[derive(Debug)]
pub enum InterpreterError {
    Lexer(LexerError),
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

        Ok(format!("{:#?}", self.lexer.tokens))
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
