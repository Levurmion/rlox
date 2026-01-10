use crate::repl::{Evaluator, EvaluatorOk, Repl};

mod repl;

struct Printer {}

impl Evaluator for Printer {
    fn eval(&mut self, input: &str) -> Result<EvaluatorOk, String> {
        if input.is_empty() {
            return Err(String::from("empty string"));
        };
        let input_lines = input.split("\n").collect::<Vec<&str>>();
        let last_input = input_lines.last().unwrap();
        if matches!(*last_input, "clear") {
            return Ok(EvaluatorOk::Clear(String::from("clearing buffer")));
        }
        return Ok(EvaluatorOk::Append(String::from(input)));
    }
}

fn main() {
    let mut printer = Printer {};
    let mut repl = Repl::new(&mut printer);
    let _ = repl.start("Welcome to my echo printer!");
}
