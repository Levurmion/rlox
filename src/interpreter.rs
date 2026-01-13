use std::collections::HashMap;

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
    InvalidIdentifier,
    ExpectedOperand,
    ExpectedExpression,
    UninitialisedVariable,
}

#[derive(Debug)]
pub enum InterpreterError {
    Compiler(CompilerError),
    Runtime(RuntimeError),
}

pub struct Interpreter {
    variables: HashMap<String, Value>,
    stack: Vec<Value>,
    ip: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
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
        match result {
            Some(Value::Number(result)) => Ok(result.to_string()),
            None => Ok("".to_string()),
            _ => todo!(),
        }
    }

    fn interpret_chunk(&mut self, chunk: &Chunk) -> Result<Option<Value>, InterpreterError> {
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
                    let constant = &chunk.constants[constant_idx];
                    self.stack.push(constant.clone());
                    self.ip += 2;
                }
                OpCode::Negate => match self.stack.pop() {
                    None => return Err(InterpreterError::Runtime(RuntimeError::ExpectedOperand)),
                    Some(operand) => {
                        self.ip += 1;
                        match operand {
                            Value::Number(operand) => self.stack.push(Value::Number(-operand)),
                            _ => todo!(),
                        }
                    }
                },
                OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide => {
                    self.interpret_binary_op(op_code)?
                }
                OpCode::SetVar => {
                    let constant_idx = chunk.code[self.ip + 1];
                    let var_name = match &chunk.constants[constant_idx] {
                        Value::String(var_name) => var_name,
                        _ => {
                            return Err(InterpreterError::Runtime(RuntimeError::InvalidIdentifier));
                        }
                    };
                    let expr_value = match self.stack.pop() {
                        Some(expr_value) => expr_value,
                        None => {
                            return Err(InterpreterError::Runtime(
                                RuntimeError::ExpectedExpression,
                            ));
                        }
                    };
                    self.variables.insert(var_name.clone(), expr_value);
                    self.ip += 2;
                }
                OpCode::GetVar => {
                    let constant_idx = chunk.code[self.ip + 1];
                    match &chunk.constants[constant_idx] {
                        Value::String(var_name) => match self.variables.get(var_name) {
                            Some(value) => self.stack.push(value.clone()),
                            None => {
                                return Err(InterpreterError::Runtime(
                                    RuntimeError::UninitialisedVariable,
                                ));
                            }
                        },
                        _ => {
                            return Err(InterpreterError::Runtime(RuntimeError::InvalidIdentifier));
                        }
                    };
                    self.ip += 2;
                }
            }
        }
        if self.stack.len() > 1 {
            return Err(InterpreterError::Runtime(
                RuntimeError::IncompleteExpression,
            ));
        }
        if self.stack.len() == 1 {
            return Ok(Some(self.stack[0].clone()));
        }
        Ok(None)
    }

    fn interpret_binary_op(&mut self, op_code: OpCode) -> Result<(), InterpreterError> {
        // pop order flipped
        let operands = (self.stack.pop(), self.stack.pop());
        let result = match operands {
            (Some(Value::Number(right)), Some(Value::Number(left))) => match op_code {
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
        self.stack.push(Value::Number(result));
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
