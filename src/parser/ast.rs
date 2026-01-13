use crate::lexer::{Token, TokenClass};

#[derive(Debug, Clone)]
pub enum ParserError {
    UnexpectedEndOfTokenStream,
    ExpectedEoF {
        token: Token,
    },
    ExpectedExpression {
        token: Token,
    },
    ExpectedOpToken {
        token: Token,
    },
    UnclosedExpression {
        token: Token,
    },
    UnexpectedToken {
        token: Token,
        expected: Option<TokenClass>,
    },
    UnexpectedUnaryOperator {
        token: Token,
    },
    UnhandledToken {
        token: Token,
    },
}

#[derive(Debug)]
pub enum AstNode {
    Empty,
    Stmt {
        token: Token,
        statement: Box<AstNode>,
    },
    VariableAssignmentStmt {
        token: Token,
        identifier: String,
        expression: Box<AstNode>,
    },
    Expr {
        token: Token,
        expr: Box<AstNode>,
    },
    BinaryExpr {
        token: Token,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    UnaryExpr {
        token: Token,
        operand: Box<AstNode>,
    },
    NumericLit {
        token: Token,
        value: f64,
    },
    VariableAccessExpr {
        token: Token,
        identifier: String,
    },
}
