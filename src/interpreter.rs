use std::{cell::Ref, fmt::format, rc::Rc, result};

use crate::{
    compiler::{Compiler, CompilerError},
    debug,
    lexer::{Lexer, LexerError, Token},
    parser::{
        ast::{AstNode, ParserError},
        parser::Parser,
    },
    repl::{Evaluator, EvaluatorOk},
};

#[derive(Debug)]
pub enum InterpreterError {
    Compiler(CompilerError),
}

pub struct Interpreter {
    compiler: Compiler,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            compiler: Compiler::new(),
        }
    }

    fn pretty_print(&self, ast: &Box<AstNode>) -> String {
        match ast.as_ref() {
            AstNode::NumericLit { value, .. } => value.to_string(),
            AstNode::BinaryExpr { op, left, right } => {
                format!(
                    "({} {} {})",
                    op.lexeme,
                    self.pretty_print(left),
                    self.pretty_print(right)
                )
            }
            AstNode::UnaryExpr { op, operand } => {
                format!("({} {})", op.lexeme, self.pretty_print(operand))
            }
            AstNode::Expr { expr, .. } => format!("[{}]", self.pretty_print(expr)),
            AstNode::Empty => String::from("Empty"),
        }
    }

    pub fn interpret(&mut self, input: String) -> Result<String, InterpreterError> {
        match self.compiler.compile(input) {
            Ok(ast) => Ok(self.pretty_print(&ast)),
            Err(err) => Err(InterpreterError::Compiler(err)),
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
