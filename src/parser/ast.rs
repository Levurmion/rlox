use std::{cell::RefCell, rc::Rc};

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ParserError {
    ExpectedEoF,
    ExpectedExpression,
    ExpectedOpToken,
    UnclosedExpression,
    UnexpectedEndOfTokenStream,
    UnexpectedToken,
    UnexpectedUnaryOperator,
    UnhandledToken,
}

#[derive(Debug)]
pub enum AstNode {
    Empty,
    ErrorNode {
        error: ParserError,
        token: Option<Token>,
    },
    Expr {
        token: Token,
        expr: Rc<RefCell<AstNode>>,
    },
    BinaryExpr {
        op: Token,
        left: Rc<RefCell<AstNode>>,
        right: Rc<RefCell<AstNode>>,
    },
    UnaryExpr {
        op: Token,
        operand: Rc<RefCell<AstNode>>,
    },
    NumericLit {
        token: Token,
        value: f64,
    },
}
