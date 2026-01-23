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

#[derive(Debug, Clone)]
pub struct Parameter {
    pub token: Token,
    pub name: String,
}

#[derive(Debug)]
pub enum AstNode {
    Empty,
    Error,
    Program {
        declarations: Vec<Box<AstNode>>,
    },

    VariableDecl {
        token: Token,
        identifier: String,
        expression: Box<AstNode>,
    },
    AssignmentDecl {
        token: Token,
        identifier: String,
        expression: Box<AstNode>,
    },
    ClassDecl {
        token: Token,
        name: String,
        fields: Vec<String>,
        constructor: Box<AstNode>,
        methods: Vec<Box<AstNode>>,
    },
    FnDecl {
        token: Token,
        name: String,
        parameters: Vec<Parameter>,
        body: Box<AstNode>,
    },

    // statements
    ExprStmt {
        token: Token,
        expression: Box<AstNode>,
    },
    PrintStmt {
        token: Token,
        expression: Box<AstNode>,
    },
    ReturnStmt {
        token: Token,
        expression: Box<AstNode>,
    },
    IfStmt {
        token: Token,
        condition: Box<AstNode>,
        if_branch: Box<AstNode>,
        else_if_branches: Vec<Box<AstNode>>,
        else_branch: Option<Box<AstNode>>,
    },
    ElseIfStmt {
        token: Token,
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    ForStmt {
        token: Token,
        initializer: Box<AstNode>,
        condition: Box<AstNode>,
        increment: Box<AstNode>,
        body: Box<AstNode>,
    },
    WhileStmt {
        token: Token,
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    Block {
        token: Token,
        declarations: Vec<Box<AstNode>>,
    },

    // expressions
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
    NilLit {
        token: Token,
    },
    IdentifierAccessExpr {
        token: Token,
        identifier: String,
    },
}
