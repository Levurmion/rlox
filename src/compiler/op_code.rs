use core::fmt;

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

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            OpCode::Constant => "Constant",
            OpCode::SetVar => "SetVar",
            OpCode::GetVar => "GetVar",
            OpCode::Print => "Print",
            OpCode::Add => "Add",
            OpCode::Subtract => "Subtract",
            OpCode::Multiply => "Multiply",
            OpCode::Divide => "Divide",
            OpCode::Negate => "Negate",
            OpCode::Equals => "Equals",
            OpCode::NotEquals => "NotEquals",
            OpCode::GreaterThan => "GreaterThan",
            OpCode::LessThan => "LessThan",
            OpCode::GreaterThanEq => "GreaterThanEq",
            OpCode::LessThanEq => "LessThanEq",
            OpCode::Not => "Not",
            OpCode::And => "And",
            OpCode::Or => "Or",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(num) => write!(f, "{}", num),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}
