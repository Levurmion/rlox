use crate::{
    interpreter::Interpreter,
    repl::{Evaluator, EvaluatorOk, Repl},
};

mod interpreter;
mod lexer;
mod parser;
mod repl;

fn main() {
    let mut interpreter = Interpreter::new();
    let mut repl = Repl::new(&mut interpreter);
    let _ = repl.start("Welcome to my interpreter!");
}
