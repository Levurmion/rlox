use crate::{
    compiler::{
        chunk::Chunk,
        compiler::{Compiler, CompilerError},
        op_code::{OpCode, Value},
    },
    repl::{Evaluator, EvaluatorOk},
};

#[derive(Debug)]
pub enum RuntimeError {
    InvalidOpCode(usize),
    InvalidBinaryOperator,
    IncompleteExpression,
    ExpectedOperand,
}

#[derive(Debug)]
pub enum InterpreterError {
    Compiler(CompilerError),
    Runtime(RuntimeError),
}

pub struct Interpreter {
    stack: Vec<Value>,
    ip: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stack: Vec::with_capacity(1024),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, input: String) -> Result<String, InterpreterError> {
        let mut compiler = Compiler::new();
        let chunk = match compiler.compile(input) {
            Err(err) => return Err(InterpreterError::Compiler(err)),
            Ok(chunk) => chunk,
        };
        let result = self.interpret_chunk(chunk)?;
        Ok(result.to_string())
    }

    fn interpret_chunk(&mut self, chunk: &Chunk) -> Result<Value, InterpreterError> {
        self.stack.clear();
        self.ip = 0;
        while self.ip < chunk.code.len() {
            let op_code = match OpCode::from_usize(chunk.code[self.ip]) {
                None => {
                    return Err(InterpreterError::Runtime(RuntimeError::InvalidOpCode(
                        chunk.code[self.ip],
                    )));
                }
                Some(op_code) => op_code,
            };

            match op_code {
                OpCode::Constant => {
                    let constant_idx = chunk.code[self.ip + 1];
                    let constant = chunk.constants[constant_idx];
                    self.stack.push(constant);
                    self.ip += 2;
                }
                OpCode::Negate => match self.stack.pop() {
                    None => return Err(InterpreterError::Runtime(RuntimeError::ExpectedOperand)),
                    Some(operand) => {
                        self.ip += 1;
                        self.stack.push(-operand);
                    }
                },
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    self.interpret_binary_op(op_code)?
                }
            }
        }
        if self.stack.len() > 1 {
            return Err(InterpreterError::Runtime(
                RuntimeError::IncompleteExpression,
            ));
        }
        Ok(self.stack[0])
    }

    fn interpret_binary_op(&mut self, op_code: OpCode) -> Result<(), InterpreterError> {
        // pop order flipped
        let operands = (self.stack.pop(), self.stack.pop());
        let result = match operands {
            (Some(right), Some(left)) => match op_code {
                OpCode::Add => left + right,
                OpCode::Subtract => left - right,
                OpCode::Divide => left / right,
                OpCode::Multiply => left * right,
                _ => {
                    return Err(InterpreterError::Runtime(
                        RuntimeError::InvalidBinaryOperator,
                    ));
                }
            },
            _ => return Err(InterpreterError::Runtime(RuntimeError::ExpectedOperand)),
        };

        self.ip += 1;
        self.stack.push(result);
        Ok(())
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
