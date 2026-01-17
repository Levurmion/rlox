#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum OpCode {
    Constant,
    SetVar,
    GetVar,
    Print,

    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,

    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanEq,
    LessThanEq,

    Not,
    And,
    Or,
}

impl OpCode {
    pub fn from_usize(byte: usize) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::Constant),
            1 => Some(OpCode::SetVar),
            2 => Some(OpCode::GetVar),
            3 => Some(OpCode::Print),
            4 => Some(OpCode::Add),
            5 => Some(OpCode::Subtract),
            6 => Some(OpCode::Multiply),
            7 => Some(OpCode::Divide),
            8 => Some(OpCode::Negate),
            9 => Some(OpCode::Equals),
            10 => Some(OpCode::NotEquals),
            11 => Some(OpCode::GreaterThan),
            12 => Some(OpCode::LessThan),
            13 => Some(OpCode::GreaterThanEq),
            14 => Some(OpCode::LessThanEq),
            15 => Some(OpCode::Not),
            16 => Some(OpCode::And),
            17 => Some(OpCode::Or),
            _ => None,
        }
    }

    pub fn to_usize(&self) -> usize {
        let value = self.clone();
        value as usize
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}
