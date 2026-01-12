use std::ops::Add;

use crate::{
    lexer::{Lexer, LexerError, OpToken, Token, TokenClass, TokenMeta},
    parser::{
        ast::{AstNode, ParserError},
        parser::Parser,
    },
};

#[derive(Debug)]
pub enum CompileError {
    UnsupportedToken,
    UnsupportedBinaryOperator,
    ExpectedOpNode,
}

#[derive(Debug)]
pub enum CompilerError {
    Lexer(LexerError),
    Parser(ParserError),
    Compiler(CompileError),
}

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

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self, input: String) -> Result<&Chunk, CompilerError> {
        let mut lexer = Lexer::new(input);
        match lexer.tokenize() {
            Err(err) => return Err(CompilerError::Lexer(err)),
            Ok(_) => {}
        }

        let mut parser = Parser::new(&lexer.tokens);
        let ast = match parser.parse() {
            Err(e) => {
                return Err(CompilerError::Parser(e));
            }
            Ok(ast) => ast,
        };

        self.chunk = Chunk::new();
        match self.compile_ast(&ast) {
            Ok(()) => Ok(&self.chunk),
            Err(err) => Err(CompilerError::Compiler(err)),
        }
    }

    fn add_instruction(&mut self, op_code: OpCode, token: &Token) {
        self.chunk.code.push(op_code.to_usize());
        self.chunk.tokens.push(token.clone());
    }

    fn add_constant(&mut self, value: &Value, token: &Token) {
        self.chunk.code.push(self.chunk.constants.len());
        self.chunk.constants.push(*value);
        self.chunk.tokens.push(token.clone());
    }

    fn compile_ast(&mut self, ast_node: &Box<AstNode>) -> Result<(), CompileError> {
        match ast_node.as_ref() {
            AstNode::Empty => {}
            AstNode::NumericLit { token, value } => {
                self.add_instruction(OpCode::Constant, token);
                self.add_constant(value, token);
            }
            AstNode::Expr { expr, .. } => {
                self.compile_ast(expr)?;
            }
            AstNode::UnaryExpr { token, operand } => {
                self.compile_ast(operand)?;

                match &token.token_class {
                    TokenClass::Op(op) => match op {
                        OpToken::Min => self.add_instruction(OpCode::Negate, token),
                        _ => return Err(CompileError::UnsupportedToken),
                    },
                    _ => return Err(CompileError::ExpectedOpNode),
                }
            }
            AstNode::BinaryExpr { token, left, right } => {
                self.compile_ast(left)?;
                self.compile_ast(right)?;

                match &token.token_class {
                    TokenClass::Op(op) => match op {
                        OpToken::Plus => self.add_instruction(OpCode::Add, token),
                        OpToken::Min => self.add_instruction(OpCode::Subtract, token),
                        OpToken::Star => self.add_instruction(OpCode::Multiply, token),
                        OpToken::Slash => self.add_instruction(OpCode::Divide, token),
                        _ => return Err(CompileError::UnsupportedBinaryOperator),
                    },
                    _ => return Err(CompileError::ExpectedOpNode),
                }
            }
        }

        Ok(())
    }
}
