#[derive(Clone, Copy)]
#[repr(usize)]
pub enum OpCode {
    Constant = 0,
    Add = 1,
    Subtract = 2,
    Multiply = 3,
    Divide = 4,
    Negate = 5,
}

impl OpCode {
    pub fn from_usize(byte: usize) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::Constant),
            1 => Some(OpCode::Add),
            2 => Some(OpCode::Subtract),
            3 => Some(OpCode::Multiply),
            4 => Some(OpCode::Divide),
            5 => Some(OpCode::Negate),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> usize {
        let value = self.clone();
        value as usize
    }
}

pub type Value = f64;
