use std::io::{self, Write};

pub enum EvaluatorOk {
    Clear(String),
    Append(String),
}
pub trait Evaluator {
    fn eval(&mut self, input: String) -> Result<EvaluatorOk, String>;
}

pub struct Repl<'a, E: Evaluator> {
    lines: String,
    stdin: io::Stdin,
    stdout: io::Stdout,
    evaluator: &'a mut E,
}

impl<'a, E: Evaluator> Repl<'a, E> {
    pub fn new(evaluator: &'a mut E) -> Repl<'a, E> {
        Repl {
            lines: String::new(),
            stdin: io::stdin(),
            stdout: io::stdout(),
            evaluator,
        }
    }

    pub fn start(&mut self, welcome_message: &str) -> io::Result<()> {
        println!("{welcome_message}");

        loop {
            print!(">> ");
            self.stdout.flush()?;

            let n = self.stdin.read_line(&mut self.lines)?;
            if n == 0 {
                break;
            }

            let input = self.lines.trim();
            if input.is_empty() {
                continue;
            }
            if matches!(input, "exit" | "kill") {
                break;
            }

            match self.evaluator.eval(input.to_string()) {
                Ok(result) => {
                    let eval_result = match result {
                        EvaluatorOk::Clear(msg) => {
                            self.lines.clear();
                            msg
                        }
                        EvaluatorOk::Append(msg) => msg,
                    };
                    println!("{eval_result}\n");
                }
                Err(error) => {
                    self.lines.clear();
                    eprintln!("eval error: {error}\n")
                }
            };
        }

        println!("Terminating REPL, bye bye!");
        Ok(())
    }
}
