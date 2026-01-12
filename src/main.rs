use crate::{interpreter::Interpreter, repl::Repl};

mod compiler;
mod interpreter;
mod lexer;
mod macros;
mod parser;
mod repl;

fn main() {
    let mut interpreter = Interpreter::new();
    let mut repl = Repl::new(&mut interpreter);
    let _ = repl.start("Welcome to my interpreter!");
}
