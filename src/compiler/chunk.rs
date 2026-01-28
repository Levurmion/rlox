use core::fmt;
use std::fmt::Error;

use crate::{
    compiler::op_code::{ConstValue, OpCode},
    lexer::tokens::Token,
};

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<usize>,
    pub constants: Vec<ConstValue>,
    pub tokens: Vec<Option<Token>>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            tokens: Vec::new(),
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = 0;
        writeln!(f, "===== Chunk =====")?;
        while i < self.code.len() {
            match OpCode::from_usize(self.code[i]) {
                None => {
                    writeln!(f, "{:04} UNKNOWN OPCODE {}", i, self.code[i])?;
                    break;
                }
                Some(op_code) => match op_code {
                    OpCode::Constant | OpCode::GetGlobalVar | OpCode::SetGlobalVar => {
                        let constant_index = self.code[i + 1];
                        writeln!(
                            f,
                            "{:04} {: <16} {:04} ({})",
                            i,
                            op_code.to_string(),
                            constant_index,
                            self.constants[constant_index]
                        )?;
                        i += 2;
                    }
                    OpCode::SetLocalVar | OpCode::GetLocalVar => {
                        let local_index = self.code[i + 1];
                        writeln!(
                            f,
                            "{:04} {: <16} {:04}",
                            i,
                            op_code.to_string(),
                            local_index,
                        )?;
                        i += 2;
                    }
                    OpCode::Jump | OpCode::JumpIfFalse => {
                        let jump_offset = self.code[i + 1];
                        writeln!(
                            f,
                            "{:04} {: <16} {:04} (to {:04})",
                            i,
                            op_code.to_string(),
                            jump_offset,
                            i + jump_offset
                        )?;
                        i += 2;
                    }
                    _ => {
                        writeln!(f, "{:04} {: <10}", i, op_code.to_string())?;
                        i += 1;
                    }
                },
            }
        }
        Ok(())
    }
}
