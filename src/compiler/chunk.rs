use crate::{compiler::op_code::Value, lexer::Token};

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<usize>,
    pub constants: Vec<Value>,
    pub tokens: Vec<Token>,
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
