use crate::{
    compiler::{
        chunk::Chunk,
        op_code::{ConstValue, OpCode},
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
pub enum CompilerError {
    UnsupportedToken,
    UnsupportedBinaryOperator,
    ExpectedOpNode,
    ExpectedReassignmentOperator,
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

    pub fn compile(&mut self, ast: &Box<AstNode>) -> Result<&Chunk, CompilerError> {
        self.chunk = Chunk::new();
        match self.compile_ast(ast) {
            Ok(_) => Ok(&self.chunk),
            Err(e) => Err(e),
        }
    }

    fn add_instruction(&mut self, op_code: OpCode, token: &Token) {
        self.chunk.code.push(op_code.to_usize());
        self.chunk.tokens.push(token.clone());
    }

    fn add_constant(&mut self, value: ConstValue, token: &Token) {
        self.chunk.code.push(self.chunk.constants.len());
        self.chunk.constants.push(value);
        self.chunk.tokens.push(token.clone());
    }

    fn compile_ast(&mut self, ast_node: &Box<AstNode>) -> Result<(), CompilerError> {
        match ast_node.as_ref() {
            AstNode::Empty => {}
            AstNode::Program { declarations } => {
                for decl in declarations {
                    self.compile_ast(decl)?;
                }
            }
            // ========== DECLARATIONS ==========
            AstNode::VariableDecl {
                token,
                identifier,
                expression,
            } => {
                self.compile_ast(expression)?;
                self.add_instruction(OpCode::SetVar, token);
                self.add_constant(ConstValue::String(identifier.clone()), token);
            }
            AstNode::AssignmentDecl {
                token,
                identifier,
                expression,
            } => match &token.token_class {
                TokenClass::Op(op) => match op {
                    OpToken::Eq => {
                        self.compile_ast(expression)?;
                        self.add_instruction(OpCode::SetVar, token);
                        self.add_constant(ConstValue::String(identifier.clone()), token);
                    }
                    OpToken::MinEq | OpToken::PlusEq | OpToken::StarEq | OpToken::SlashEq => {
                        // First, get the current value of the variable
                        self.add_instruction(OpCode::GetVar, token);
                        self.add_constant(ConstValue::String(identifier.clone()), token);

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
                        self.add_constant(ConstValue::String(identifier.clone()), token);
                    }
                    _ => return Err(CompilerError::ExpectedReassignmentOperator),
                },
                _ => return Err(CompilerError::ExpectedReassignmentOperator),
            },

            // ========== STATEMENTS ==========
            AstNode::PrintStmt { token, expression } => {
                self.compile_ast(expression)?;
                self.add_instruction(OpCode::Print, token);
            }
            AstNode::ReturnStmt { token, expression } => {
                self.compile_ast(expression)?;
                self.add_instruction(OpCode::Return, token);
            }
            AstNode::ExprStmt { token, expression } => {
                self.compile_ast(expression)?;
                self.add_instruction(OpCode::Pop, token);
            }

            AstNode::NumericLit { token, value } => {
                self.add_instruction(OpCode::Constant, token);
                self.add_constant(ConstValue::Number(*value), token);
            }
            AstNode::StringLit { token, value } => {
                self.add_instruction(OpCode::Constant, token);
                self.add_constant(ConstValue::String(value.clone().replace("\"", "")), token);
            }
            AstNode::BooleanLit { token, value } => {
                self.add_instruction(OpCode::Constant, token);
                self.add_constant(ConstValue::Boolean(*value), token);
            }
            AstNode::UnaryExpr { token, operand } => {
                self.compile_ast(operand)?;

                match &token.token_class {
                    TokenClass::Op(op) => match op {
                        OpToken::Min => self.add_instruction(OpCode::Negate, token),
                        OpToken::Bang => self.add_instruction(OpCode::Not, token),
                        _ => return Err(CompilerError::UnsupportedToken),
                    },
                    _ => return Err(CompilerError::ExpectedOpNode),
                }
            }
            AstNode::IdentifierAccessExpr { token, identifier } => {
                self.add_instruction(OpCode::GetVar, token);
                self.add_constant(ConstValue::String(identifier.clone()), token);
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
                        _ => return Err(CompilerError::UnsupportedBinaryOperator),
                    },
                    _ => return Err(CompilerError::ExpectedOpNode),
                }
            }
            _ => todo!(),
        }

        Ok(())
    }
}
