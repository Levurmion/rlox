use core::panic;
use std::{collections::HashSet, hash::Hash};

use crate::{
    compiler::{
        chunk::Chunk,
        op_code::{ConstValue, OpCode},
    },
    debug,
    lexer::tokens::{OpToken, Token, TokenClass},
    parser::ast::AstNode,
};

#[derive(Debug)]
pub enum CompilerError {
    UnsupportedToken,
    UnsupportedBinaryOperator,
    ExpectedOpNode,
    ExpectedReassignmentOperator,
    UninitialisedVariable,
    CannotRedeclareVariableInScope(String),
}

#[derive(Debug)]
struct Local {
    name: String,
    depth: usize,
}

pub struct Compiler {
    globals: HashSet<String>,
    locals: Vec<Local>,
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            globals: HashSet::new(),
            locals: Vec::new(),
            chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self, ast: &Box<AstNode>) -> Result<&Chunk, CompilerError> {
        self.chunk = Chunk::new();
        match self.compile_ast(ast, 0) {
            Ok(_) => Ok(&self.chunk),
            Err(e) => Err(e),
        }
    }

    fn add_instruction(&mut self, op_code: OpCode, token: Option<Token>) {
        self.chunk.code.push(op_code.to_usize());
        self.chunk.tokens.push(token);
    }

    fn add_constant(&mut self, value: ConstValue, token: Option<Token>) {
        self.chunk.code.push(self.chunk.constants.len());
        self.chunk.constants.push(value);
        self.chunk.tokens.push(token);
    }

    fn add_local(&mut self, name: String, depth: usize) -> usize {
        self.locals.push(Local { name, depth });
        self.locals.len() - 1 // return the local variable index on the stack
    }

    fn resolve_local(&self, name: &str) -> Option<(usize, usize)> {
        if self.locals.len() == 0 {
            return None;
        }
        let mut i = self.locals.len() - 1;
        loop {
            // find a local variable with the given name, return its index on the stack
            if self.locals[i].name == name {
                return Some((i, self.locals[i].depth));
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        None
    }

    fn compile_set_local_variable(&mut self, name: &str) {
        match self.resolve_local(name) {
            Some((local_index, ..)) => {
                self.add_instruction(OpCode::SetLocalVar, None);
                self.chunk.code.push(local_index);
                self.chunk.tokens.push(None);
            }
            None => {
                // then probably in global scope
                self.add_instruction(OpCode::SetGlobalVar, None);
                self.add_constant(ConstValue::String(name.to_string()), None);
            }
        }
    }

    fn compile_block(
        &mut self,
        declarations: &Vec<Box<AstNode>>,
        scope_depth: usize,
    ) -> Result<(), CompilerError> {
        for decl in declarations {
            self.compile_ast(decl, scope_depth + 1)?;
        }

        if self.locals.len() <= 0 {
            return Ok(());
        }

        let mut i = self.locals.len() - 1;
        loop {
            if self.locals[i].depth > scope_depth {
                self.locals.pop();
                self.add_instruction(OpCode::Pop, None);
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }

        Ok(())
    }

    fn compile_ast(
        &mut self,
        ast_node: &Box<AstNode>,
        scope_depth: usize,
    ) -> Result<(), CompilerError> {
        match ast_node.as_ref() {
            AstNode::Empty => {}
            AstNode::Program { declarations } => {
                for decl in declarations {
                    self.compile_ast(decl, 0)?;
                }
            }
            // ========== DECLARATIONS ==========
            AstNode::VariableDecl {
                token,
                identifier,
                expression,
            } => {
                self.compile_ast(expression, scope_depth)?;
                if scope_depth == 0 {
                    match self.globals.get(identifier) {
                        Some(_) => {
                            // variable already declared
                            return Err(CompilerError::CannotRedeclareVariableInScope(
                                identifier.clone(),
                            ));
                        }
                        None => {
                            self.globals.insert(identifier.clone());
                            self.add_instruction(OpCode::SetGlobalVar, Some(token.clone()));
                            self.add_constant(
                                ConstValue::String(identifier.clone()),
                                Some(token.clone()),
                            );
                        }
                    }
                } else {
                    match self.resolve_local(identifier) {
                        Some((_, var_depth)) if var_depth == scope_depth => {
                            // variable already declared in this scope
                            return Err(CompilerError::CannotRedeclareVariableInScope(
                                identifier.clone(),
                            ));
                        }
                        _ => {
                            self.add_instruction(OpCode::SetLocalVar, Some(token.clone()));
                            let local_index = self.add_local(identifier.clone(), scope_depth);
                            self.chunk.code.push(local_index);
                            self.chunk.tokens.push(Some(token.clone()));
                        }
                    }
                }
            }
            AstNode::AssignmentDecl {
                token,
                identifier,
                expression,
            } => match &token.token_class {
                TokenClass::Op(op) => match op {
                    OpToken::Eq => {
                        if scope_depth == 0 {
                            self.compile_ast(expression, scope_depth)?;
                            self.add_instruction(OpCode::SetGlobalVar, Some(token.clone()));
                            self.add_constant(
                                ConstValue::String(identifier.clone()),
                                Some(token.clone()),
                            );
                        } else {
                            self.compile_ast(expression, scope_depth)?;
                            self.compile_set_local_variable(&identifier);
                        }
                    }
                    OpToken::MinEq | OpToken::PlusEq | OpToken::StarEq | OpToken::SlashEq => {
                        // First, get the current value of the variable
                        self.add_instruction(OpCode::GetGlobalVar, Some(token.clone()));
                        self.add_constant(
                            ConstValue::String(identifier.clone()),
                            Some(token.clone()),
                        );

                        // Then, compile the new expression
                        self.compile_ast(expression, scope_depth)?;

                        // Perform the appropriate operation
                        match op {
                            OpToken::PlusEq => {
                                self.add_instruction(OpCode::Add, Some(token.clone()))
                            }
                            OpToken::MinEq => {
                                self.add_instruction(OpCode::Subtract, Some(token.clone()))
                            }
                            OpToken::StarEq => {
                                self.add_instruction(OpCode::Multiply, Some(token.clone()))
                            }
                            OpToken::SlashEq => {
                                self.add_instruction(OpCode::Divide, Some(token.clone()))
                            }
                            _ => unreachable!(),
                        }

                        // Finally, set the variable with the new value
                        if scope_depth == 0 {
                            self.add_instruction(OpCode::SetGlobalVar, Some(token.clone()));
                            self.add_constant(
                                ConstValue::String(identifier.clone()),
                                Some(token.clone()),
                            );
                            return Ok(());
                        } else {
                            self.compile_set_local_variable(&identifier);
                        }
                    }
                    _ => return Err(CompilerError::ExpectedReassignmentOperator),
                },
                _ => return Err(CompilerError::ExpectedReassignmentOperator),
            },

            // ========== STATEMENTS ==========
            AstNode::PrintStmt { token, expression } => {
                self.compile_ast(expression, scope_depth)?;
                self.add_instruction(OpCode::Print, Some(token.clone()));
            }
            AstNode::ReturnStmt { token, expression } => {
                self.compile_ast(expression, scope_depth)?;
                self.add_instruction(OpCode::Return, Some(token.clone()));
            }
            AstNode::ExprStmt { token, expression } => {
                self.compile_ast(expression, scope_depth)?;
                self.add_instruction(OpCode::Pop, Some(token.clone()));
            }
            AstNode::IfStmt {
                token,
                condition,
                then_branch,
                else_branch,
            } => {
                // then branch
                self.compile_ast(condition, scope_depth)?;
                let if_false_jump_offset_placeholder_idx =
                    self.emit_jump(OpCode::JumpIfFalse, Some(token.clone()));
                self.add_instruction(OpCode::Pop, None); // pop condition result
                self.compile_ast(then_branch, scope_depth)?;

                // else branch
                if let Some(else_branch_node) = else_branch {
                    // prepare unconditional jump statement to skip the else branch
                    let if_true_jump_placeholder_idx =
                        self.emit_jump(OpCode::Jump, Some(token.clone()));
                    self.add_instruction(OpCode::Pop, None); // pop condition result
                    // backpatch if jump to start of else branch
                    self.backpatch_jump(if_false_jump_offset_placeholder_idx);
                    self.compile_ast(else_branch_node, scope_depth)?;
                    // backpatch unconditional jump to after else branch
                    self.backpatch_jump(if_true_jump_placeholder_idx);
                } else {
                    // backpatch if jump to after then branch
                    self.backpatch_jump(if_false_jump_offset_placeholder_idx);
                }
            }

            // ========== LITERALS ==========
            AstNode::NumericLit { token, value } => {
                self.add_instruction(OpCode::Constant, Some(token.clone()));
                self.add_constant(ConstValue::Number(*value), Some(token.clone()));
            }
            AstNode::StringLit { token, value } => {
                self.add_instruction(OpCode::Constant, Some(token.clone()));
                self.add_constant(
                    ConstValue::String(value.clone().replace("\"", "")),
                    Some(token.clone()),
                );
            }
            AstNode::BooleanLit { token, value } => {
                self.add_instruction(OpCode::Constant, Some(token.clone()));
                self.add_constant(ConstValue::Boolean(*value), Some(token.clone()));
            }
            AstNode::UnaryExpr { token, operand } => {
                self.compile_ast(operand, scope_depth)?;

                match &token.token_class {
                    TokenClass::Op(op) => match op {
                        OpToken::Min => self.add_instruction(OpCode::Negate, Some(token.clone())),
                        OpToken::Bang => self.add_instruction(OpCode::Not, Some(token.clone())),
                        _ => return Err(CompilerError::UnsupportedToken),
                    },
                    _ => return Err(CompilerError::ExpectedOpNode),
                }
            }
            AstNode::IdentifierAccessExpr { token, identifier } => {
                if scope_depth == 0 {
                    self.add_instruction(OpCode::GetGlobalVar, Some(token.clone()));
                    self.add_constant(ConstValue::String(identifier.clone()), Some(token.clone()));
                    return Ok(());
                }
                match self.resolve_local(identifier) {
                    Some((local_index, ..)) => {
                        self.add_instruction(OpCode::GetLocalVar, Some(token.clone()));
                        self.chunk.code.push(local_index);
                        self.chunk.tokens.push(Some(token.clone()));
                    }
                    None => {
                        self.add_instruction(OpCode::GetGlobalVar, Some(token.clone()));
                        self.add_constant(
                            ConstValue::String(identifier.clone()),
                            Some(token.clone()),
                        );
                    }
                }
            }
            AstNode::BinaryExpr { token, left, right } => {
                self.compile_ast(left, scope_depth)?;
                self.compile_ast(right, scope_depth)?;

                match &token.token_class {
                    TokenClass::Op(op) => match op {
                        OpToken::Plus => self.add_instruction(OpCode::Add, Some(token.clone())),
                        OpToken::Min => self.add_instruction(OpCode::Subtract, Some(token.clone())),
                        OpToken::Star => {
                            self.add_instruction(OpCode::Multiply, Some(token.clone()))
                        }
                        OpToken::Slash => self.add_instruction(OpCode::Divide, Some(token.clone())),
                        OpToken::EqEq => self.add_instruction(OpCode::Equals, Some(token.clone())),
                        OpToken::BangEq => {
                            self.add_instruction(OpCode::NotEquals, Some(token.clone()))
                        }
                        OpToken::Gt => {
                            self.add_instruction(OpCode::GreaterThan, Some(token.clone()))
                        }
                        OpToken::Lt => self.add_instruction(OpCode::LessThan, Some(token.clone())),
                        OpToken::Geq => {
                            self.add_instruction(OpCode::GreaterThanEq, Some(token.clone()))
                        }
                        OpToken::Leq => {
                            self.add_instruction(OpCode::LessThanEq, Some(token.clone()))
                        }
                        OpToken::And => self.add_instruction(OpCode::And, Some(token.clone())),
                        OpToken::Or => self.add_instruction(OpCode::Or, Some(token.clone())),
                        _ => return Err(CompilerError::UnsupportedBinaryOperator),
                    },
                    _ => return Err(CompilerError::ExpectedOpNode),
                }
            }
            AstNode::Block { declarations, .. } => self.compile_block(declarations, scope_depth)?,
            _ => todo!(),
        }

        Ok(())
    }

    fn emit_jump(&mut self, op_code: OpCode, token: Option<Token>) -> usize {
        self.add_instruction(op_code, token);
        // placeholder for jump offset
        self.chunk.code.push(0);
        self.chunk.tokens.push(None);
        self.chunk.code.len() - 1
    }

    fn backpatch_jump(&mut self, jump_placeholder_idx: usize) {
        let jump_offset = self.chunk.code.len() - jump_placeholder_idx;
        self.chunk.code[jump_placeholder_idx] = jump_offset + 1;
    }
}
