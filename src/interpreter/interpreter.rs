use std::collections::HashMap;

use crate::{
    compiler::{
        chunk::Chunk,
        compiler::{Compiler, CompilerError},
        op_code::{ConstValue, OpCode},
    },
    interpreter::values::LoxValue,
    repl::{Evaluator, EvaluatorOk, EvaluatorOp},
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
    ExpectedNumberType,
    ExpectedBooleanType,
    InvalidBinaryOperation,
}

#[derive(Debug)]
pub enum InterpreterError {
    Compiler(CompilerError),
    Runtime((RuntimeError, Chunk)),
}

pub struct Interpreter {
    stdout: Option<String>,
    variables: HashMap<String, LoxValue>,
    stack: Vec<LoxValue>,
    ip: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            stdout: None,
            variables: HashMap::new(),
            stack: Vec::with_capacity(1024),
            ip: 0,
        }
    }

    pub fn interpret(&mut self, input: String) -> Result<Option<String>, InterpreterError> {
        self.stdout = None;
        let mut compiler = Compiler::new();
        let chunk = match compiler.compile(input) {
            Err(err) => return Err(InterpreterError::Compiler(err)),
            Ok(chunk) => chunk,
        };
        match self.interpret_chunk(chunk) {
            Err(err) => return Err(InterpreterError::Runtime((err, chunk.clone()))),
            Ok(()) => match &self.stdout {
                Some(stdout) => Ok(Some(stdout.clone())),
                None => Ok(None),
            },
        }
    }

    fn interpret_chunk(&mut self, chunk: &Chunk) -> Result<(), RuntimeError> {
        self.stack.clear();
        self.ip = 0;
        while self.ip < chunk.code.len() {
            let op_code = match OpCode::from_usize(chunk.code[self.ip]) {
                None => {
                    return Err(RuntimeError::InvalidOpCode(chunk.code[self.ip]));
                }
                Some(op_code) => op_code,
            };

            match op_code {
                OpCode::Print => self.interpret_print()?,
                OpCode::Negate => match self.stack.pop() {
                    None => return Err(RuntimeError::ExpectedOperand),
                    Some(operand) => {
                        match operand {
                            LoxValue::Number(operand) => {
                                self.stack.push(LoxValue::Number(-operand))
                            }
                            _ => {
                                return Err(RuntimeError::ExpectedNumberType);
                            }
                        }
                        self.ip += 1;
                    }
                },
                OpCode::Not => match self.stack.pop() {
                    None => return Err(RuntimeError::ExpectedOperand),
                    Some(operand) => {
                        match operand {
                            LoxValue::Boolean(operand) => {
                                self.stack.push(LoxValue::Boolean(!operand))
                            }
                            _ => {
                                return Err(RuntimeError::ExpectedBooleanType);
                            }
                        }
                        self.ip += 1;
                    }
                },
                OpCode::SetVar => {
                    self.interpret_instruction_with_constant(
                        chunk,
                        |vm, constant| match constant {
                            ConstValue::String(var_name) => {
                                let expr_value = match vm.stack.pop() {
                                    Some(expr_value) => expr_value,
                                    None => {
                                        return Err(RuntimeError::ExpectedExpression);
                                    }
                                };
                                vm.variables.insert(var_name.clone(), expr_value);
                                Ok(())
                            }
                            _ => Err(RuntimeError::InvalidIdentifier),
                        },
                    )?;
                }
                OpCode::GetVar => {
                    self.interpret_instruction_with_constant(chunk, |vm, constant| match constant {
                        ConstValue::String(var_name) => match vm.variables.get(var_name) {
                            Some(value) => {
                                vm.stack.push(value.clone());
                                Ok(())
                            }
                            None => Err(RuntimeError::UninitialisedVariable),
                        },
                        _ => Err(RuntimeError::InvalidIdentifier),
                    })
                }?,
                OpCode::Constant => {
                    self.interpret_instruction_with_constant(chunk, |vm, constant| {
                        let lox_value = match constant {
                            ConstValue::Number(num) => LoxValue::Number(*num),
                            ConstValue::String(s) => LoxValue::String(s.clone()),
                            ConstValue::Boolean(b) => LoxValue::Boolean(*b),
                        };
                        vm.stack.push(lox_value);
                        Ok(())
                    })?
                }
                OpCode::Add
                | OpCode::Subtract
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Equals
                | OpCode::NotEquals
                | OpCode::GreaterThan
                | OpCode::LessThan
                | OpCode::GreaterThanEq
                | OpCode::LessThanEq
                | OpCode::And
                | OpCode::Or => self.interpret_binary_op(op_code)?,
            }
        }
        if self.stack.len() > 1 {
            return Err(RuntimeError::IncompleteExpression);
        }
        Ok(())
    }

    fn interpret_instruction_with_constant(
        &mut self,
        chunk: &Chunk,
        func: impl Fn(&mut Self, &ConstValue) -> Result<(), RuntimeError>,
    ) -> Result<(), RuntimeError> {
        let constant_idx = chunk.code[self.ip + 1];
        let constant = &chunk.constants[constant_idx];
        func(self, constant)?;
        self.ip += 2;
        Ok(())
    }

    fn interpret_binary_op(&mut self, op_code: OpCode) -> Result<(), RuntimeError> {
        // pop order flipped
        let operands = (self.stack.pop(), self.stack.pop());
        let result = match operands {
            (Some(LoxValue::Number(right)), Some(LoxValue::Number(left))) => match op_code {
                OpCode::Add => LoxValue::Number(left + right),
                OpCode::Subtract => LoxValue::Number(left - right),
                OpCode::Divide => LoxValue::Number(left / right),
                OpCode::Multiply => LoxValue::Number(left * right),
                OpCode::Equals => LoxValue::Boolean(left == right),
                OpCode::NotEquals => LoxValue::Boolean(left != right),
                OpCode::GreaterThan => LoxValue::Boolean(left > right),
                OpCode::LessThan => LoxValue::Boolean(left < right),
                OpCode::GreaterThanEq => LoxValue::Boolean(left >= right),
                OpCode::LessThanEq => LoxValue::Boolean(left <= right),
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperator);
                }
            },
            (Some(LoxValue::String(right)), Some(LoxValue::String(left))) => match op_code {
                OpCode::Add => {
                    let mut combined = left;
                    combined.push_str(&right);
                    LoxValue::String(combined)
                }
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperator);
                }
            },
            (Some(LoxValue::Boolean(right)), Some(LoxValue::Boolean(left))) => match op_code {
                OpCode::And => LoxValue::Boolean(left && right),
                OpCode::Or => LoxValue::Boolean(left || right),
                OpCode::Equals => LoxValue::Boolean(left == right),
                OpCode::NotEquals => LoxValue::Boolean(left != right),
                _ => {
                    return Err(RuntimeError::InvalidBinaryOperator);
                }
            },
            (None, _) | (_, None) => {
                return Err(RuntimeError::ExpectedOperand);
            }
            _ => {
                return Err(RuntimeError::InvalidBinaryOperation);
            }
        };

        self.ip += 1;
        self.stack.push(result);
        Ok(())
    }

    fn interpret_print(&mut self) -> Result<(), RuntimeError> {
        let to_print = match self.stack.pop() {
            None => {
                return Err(RuntimeError::ExpectedExpression);
            }
            Some(value) => value,
        };
        match to_print {
            LoxValue::Number(num) => {
                self.stdout = Some(num.to_string());
            }
            LoxValue::String(s) => {
                self.stdout = Some(s);
            }
            LoxValue::Boolean(b) => {
                self.stdout = Some(b.to_string());
            }
            LoxValue::Object(_) => todo!(),
        }
        self.ip += 1;
        Ok(())
    }
}

impl Evaluator for Interpreter {
    fn eval(&mut self, input: String) -> Result<EvaluatorOk, String> {
        let interpret_result = self.interpret(input);
        match interpret_result {
            Ok(result) => match result {
                Some(to_print) => Ok(EvaluatorOk::Clear(EvaluatorOp::Print(to_print))),
                None => Ok(EvaluatorOk::Clear(EvaluatorOp::None)),
            },
            Err(err) => match err {
                InterpreterError::Runtime((runtime_error, chunk)) => {
                    Err(format!("{:#?} \n\n{}", runtime_error, chunk))
                }
                InterpreterError::Compiler(compiler_error) => Err(format!("{:#?}", compiler_error)),
            },
        }
    }
}
