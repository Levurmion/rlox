use crate::{interpreter::interpreter::Interpreter, repl::Repl};
use clap::Parser;

mod compiler;
mod interpreter;
mod lexer;
mod macros;
mod parser;
mod repl;

#[derive(clap::Parser, Debug)]
#[command(name = "rlox", version = "0.0.1", about = "A Lox interpreter in Rust")]
struct Cli {
    #[arg(short, long, help = "Run in REPL mode")]
    repl: bool,

    #[arg(short, long, help = "Input file to execute")]
    input: Option<String>,
}

fn main() {
    let cli_args = Cli::parse();

    if cli_args.repl {
        let mut interpreter = Interpreter::new();
        let mut repl = Repl::new(&mut interpreter);
        let _ = repl.start("Welcome to my interpreter!");
    } else if cli_args.input.is_some() {
        let input_file_path = cli_args.input.unwrap();
        let input_file =
            std::fs::read_to_string(input_file_path).expect("Failed to read input file");

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(input_file);
        match result {
            Ok(output) => {
                if let Some(out) = output {
                    println!("{}", out);
                }
            }
            Err(err) => match err {
                interpreter::interpreter::InterpreterError::Runtime((runtime_error, chunk)) => {
                    eprintln!(
                        "Runtime Error: {:#?} \n\n{}",
                        runtime_error,
                        chunk.to_string()
                    );
                }
                err => {
                    eprintln!("Error: {:#?}", err);
                }
            },
        }
    } else {
        eprintln!(
            "Please provide either --repl to start the REPL or --input <file> to run a file."
        );
    }
}
