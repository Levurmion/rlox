use crate::lexer::{tokens::Token, tokens::TokenClass};

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
    ExpectedReassignmentOperator {
        token: Token,
    },
    UnclosedExpression {
        token: Token,
    },
    UnexpectedToken {
        token: Token,
        expected: Option<TokenClass>,
    },
    UnexpectedKeyword {
        token: Token,
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
    PrintStmt {
        token: Token,
        expression: Box<AstNode>,
    },
    VariableDecl {
        token: Token,
        identifier: String,
        expression: Box<AstNode>,
    },
    VariableReassignDecl {
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
    StringLit {
        token: Token,
        value: String,
    },
    BooleanLit {
        token: Token,
        value: bool,
    },
    VariableAccessExpr {
        token: Token,
        identifier: String,
    },
}
