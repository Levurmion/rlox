use core::fmt;

#[derive(Debug, Clone, Copy)]
#[repr(usize)]
pub enum OpCode {
    Constant,
    SetGlobalVar, // CONST for variable name
    GetGlobalVar, // CONST for variable name
    SetLocalVar,  // CONST for local variable position on stack
    GetLocalVar,  // CONST for local variable position on stack

    Print,
    Return,

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

    Pop,
    Jump,
}

impl OpCode {
    pub fn from_usize(byte: usize) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::Constant),
            1 => Some(OpCode::SetGlobalVar),
            2 => Some(OpCode::GetGlobalVar),
            3 => Some(OpCode::SetLocalVar),
            4 => Some(OpCode::GetLocalVar),
            5 => Some(OpCode::Print),
            6 => Some(OpCode::Return),
            7 => Some(OpCode::Add),
            8 => Some(OpCode::Subtract),
            9 => Some(OpCode::Multiply),
            10 => Some(OpCode::Divide),
            11 => Some(OpCode::Negate),
            12 => Some(OpCode::Equals),
            13 => Some(OpCode::NotEquals),
            14 => Some(OpCode::GreaterThan),
            15 => Some(OpCode::LessThan),
            16 => Some(OpCode::GreaterThanEq),
            17 => Some(OpCode::LessThanEq),
            18 => Some(OpCode::Not),
            19 => Some(OpCode::And),
            20 => Some(OpCode::Or),
            21 => Some(OpCode::Pop),
            22 => Some(OpCode::Jump),
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
            OpCode::SetGlobalVar => "SetGlobalVar",
            OpCode::GetGlobalVar => "GetGlobalVar",
            OpCode::SetLocalVar => "SetLocalVar",
            OpCode::GetLocalVar => "GetLocalVar",
            OpCode::Print => "Print",
            OpCode::Return => "Return",
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
            OpCode::Pop => "Pop",
            OpCode::Jump => "Jump",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub enum ConstValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstValue::Number(num) => write!(f, "{}", num),
            ConstValue::String(s) => write!(f, "\"{}\"", s),
            ConstValue::Boolean(b) => write!(f, "{}", b),
        }
    }
}
