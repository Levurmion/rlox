use std::{cell::RefCell, rc::Rc};

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedEndOfTokenStream,
    ExpectedEoF { token: Token },
    ExpectedExpression { token: Token },
    ExpectedOpToken { token: Token },
    UnclosedExpression { token: Token },
    UnexpectedToken { token: Token },
    UnexpectedUnaryOperator { token: Token },
    UnhandledToken { token: Token },
}

#[derive(Debug)]
pub enum AstNode {
    Empty,
    Expr {
        token: Token,
        expr: Box<AstNode>,
    },
    BinaryExpr {
        op: Token,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryExpr {
        op: Token,
        operand: Box<AstNode>,
    },
    NumericLit {
        token: Token,
        value: f64,
    },
}
