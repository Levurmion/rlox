use crate::{
    compiler::{
        chunk::Chunk,
        op_code::{OpCode, Value},
    },
    debug,
    lexer::{
        lexer::{Lexer, LexerError},
        tokens::{OpToken, Token, TokenClass},
    },
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
    ExpectedReassignmentOperator,
}

#[derive(Debug)]
pub enum CompilerError {
    Lexer(LexerError),
    Parser(ParserError),
    Compiler(CompileError),
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

    fn add_constant(&mut self, value: Value, token: &Token) {
        self.chunk.code.push(self.chunk.constants.len());
        self.chunk.constants.push(value);
        self.chunk.tokens.push(token.clone());
    }

    fn compile_ast(&mut self, ast_node: &Box<AstNode>) -> Result<(), CompileError> {
        match ast_node.as_ref() {
            AstNode::Empty => {}
            AstNode::NumericLit { token, value } => {
                self.add_instruction(OpCode::Constant, token);
                self.add_constant(Value::Number(*value), token);
            }
            AstNode::StringLit { token, value } => {
                self.add_instruction(OpCode::Constant, token);
                self.add_constant(Value::String(value.clone().replace("\"", "")), token);
            }
            AstNode::BooleanLit { token, value } => {
                self.add_instruction(OpCode::Constant, token);
                self.add_constant(Value::Boolean(*value), token);
            }
            AstNode::Expr { expr, .. } => {
                self.compile_ast(expr)?;
            }
            AstNode::UnaryExpr { token, operand } => {
                self.compile_ast(operand)?;

                match &token.token_class {
                    TokenClass::Op(op) => match op {
                        OpToken::Min => self.add_instruction(OpCode::Negate, token),
                        OpToken::Bang => self.add_instruction(OpCode::Not, token),
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
                        OpToken::EqEq => self.add_instruction(OpCode::Equals, token),
                        OpToken::BangEq => self.add_instruction(OpCode::NotEquals, token),
                        OpToken::Gt => self.add_instruction(OpCode::GreaterThan, token),
                        OpToken::Lt => self.add_instruction(OpCode::LessThan, token),
                        OpToken::Geq => self.add_instruction(OpCode::GreaterThanEq, token),
                        OpToken::Leq => self.add_instruction(OpCode::LessThanEq, token),
                        OpToken::And => self.add_instruction(OpCode::And, token),
                        OpToken::Or => self.add_instruction(OpCode::Or, token),
                        _ => return Err(CompileError::UnsupportedBinaryOperator),
                    },
                    _ => return Err(CompileError::ExpectedOpNode),
                }
            }
            AstNode::Stmt { statement, .. } => self.compile_ast(statement)?,
            AstNode::VariableDeclarationStmt {
                token,
                identifier,
                expression,
            } => {
                self.compile_ast(expression)?;
                self.add_instruction(OpCode::SetVar, token);
                self.add_constant(Value::String(identifier.clone()), token);
            }
            AstNode::VariableAccessExpr { token, identifier } => {
                self.add_instruction(OpCode::GetVar, token);
                self.add_constant(Value::String(identifier.clone()), token);
            }
            AstNode::VariableReassignmentStmt {
                token,
                identifier,
                expression,
            } => match &token.token_class {
                TokenClass::Op(op) => match op {
                    OpToken::Eq => {
                        self.compile_ast(expression)?;
                        self.add_instruction(OpCode::SetVar, token);
                        self.add_constant(Value::String(identifier.clone()), token);
                    }
                    OpToken::MinEq | OpToken::PlusEq | OpToken::StarEq | OpToken::SlashEq => {
                        // First, get the current value of the variable
                        self.add_instruction(OpCode::GetVar, token);
                        self.add_constant(Value::String(identifier.clone()), token);

                        // Then, compile the new expression
                        self.compile_ast(expression)?;

                        // Perform the appropriate operation
                        match op {
                            OpToken::PlusEq => self.add_instruction(OpCode::Add, token),
                            OpToken::MinEq => self.add_instruction(OpCode::Subtract, token),
                            OpToken::StarEq => self.add_instruction(OpCode::Multiply, token),
                            OpToken::SlashEq => self.add_instruction(OpCode::Divide, token),
                            _ => unreachable!(),
                        }

                        // Finally, set the variable with the new value
                        self.add_instruction(OpCode::SetVar, token);
                        self.add_constant(Value::String(identifier.clone()), token);
                    }
                    _ => return Err(CompileError::ExpectedReassignmentOperator),
                },
                _ => return Err(CompileError::ExpectedReassignmentOperator),
            },
            AstNode::PrintStmt { token, expression } => {
                self.compile_ast(expression)?;
                self.add_instruction(OpCode::Print, token);
            }
        }

        Ok(())
    }
}
