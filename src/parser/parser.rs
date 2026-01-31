use core::panic;
use std::{clone, f32::consts::E};

use crate::{
    debug,
    lexer::tokens::{AtomToken, DelimToken, KeywordToken, OpToken, Token, TokenClass},
    parser::ast::{AstNode, Parameter, ParserError},
};

pub type ParseResult = Result<Box<AstNode>, ()>;

fn infix_bp(op: &OpToken) -> Option<(f32, f32)> {
    match op {
        // infix arithmetic
        OpToken::Star => Some((10.1, 10.0)),
        OpToken::Slash => Some((9.1, 9.0)),
        OpToken::Min => Some((8.1, 11.0)),
        OpToken::Plus => Some((7.1, 7.0)),

        // comparison
        OpToken::Gt
        | OpToken::Geq
        | OpToken::Lt
        | OpToken::Leq
        | OpToken::EqEq
        | OpToken::BangEq => Some((6.1, 6.0)),

        // boolean
        OpToken::And => Some((5.1, 5.0)),
        OpToken::Or => Some((4.1, 4.0)),
        OpToken::Bang => Some((3.1, 3.0)),

        _ => None,
    }
}

pub struct Parser<'a> {
    errors: Vec<ParserError>,
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            errors: Vec::new(),
            tokens,
            pos: 0,
        }
    }

    // utilities
    fn peek(&mut self) -> Result<Token, ()> {
        match self.tokens.get(self.pos) {
            Some(token) => Ok(token.clone()),
            None => {
                self.errors.push(ParserError::UnexpectedEndOfTokenStream);
                Err(())
            }
        }
    }

    fn lookahead(&mut self, k: usize) -> Result<Token, ()> {
        match self.tokens.get(self.pos + k) {
            Some(token) => Ok(token.clone()),
            None => {
                self.errors.push(ParserError::UnexpectedEndOfTokenStream);
                Err(())
            }
        }
    }

    fn consume(&mut self) -> Result<Token, ()> {
        let consumed = self.peek()?;
        self.pos += 1;
        Ok(consumed)
    }

    fn consume_expecting(&mut self, expected: TokenClass) -> Result<Token, ()> {
        let token = self.peek()?;
        if token.token_class == expected {
            return self.consume();
        }
        self.errors.push(ParserError::UnexpectedToken {
            token,
            expected: Some(expected.clone()),
        });
        Err(())
    }

    fn recover(&mut self, to_token: TokenClass) {
        loop {
            match self.peek() {
                Ok(token)
                    if token.token_class == to_token
                        || token.token_class == TokenClass::Delim(DelimToken::EoF) =>
                {
                    break;
                }
                Err(_) => break,
                _ => self.pos += 1,
            }
        }
    }

    // parsing
    pub fn parse(&mut self) -> Result<Box<AstNode>, Vec<ParserError>> {
        let ast = self.parse_program().unwrap();
        debug!(ast);
        if self.errors.is_empty() {
            Ok(ast)
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_program(&mut self) -> ParseResult {
        let mut declarations: Vec<Box<AstNode>> = Vec::new();
        loop {
            match self.peek() {
                Err(_) => {
                    break;
                }
                Ok(token) => match token.token_class {
                    TokenClass::Delim(DelimToken::EoF) => break,
                    _ => match self.parse_decl() {
                        Ok(decl) => declarations.push(decl),
                        Err(_) => {
                            declarations.push(Box::new(AstNode::Error));
                            self.recover(TokenClass::Delim(DelimToken::Semicolon));
                        }
                    },
                },
            }
        }
        Ok(Box::new(AstNode::Program { declarations }))
    }

    fn parse_decl(&mut self) -> ParseResult {
        let token = self.peek()?;
        let statement = match &token.token_class {
            TokenClass::Keyword(keyword) => match keyword {
                // declarations
                KeywordToken::Let => self.parse_variable_decl()?,
                KeywordToken::Class => self.parse_class_decl()?,
                KeywordToken::Fn => self.parse_function_decl()?,
                _ => self.parse_stmt()?,
            },
            TokenClass::Delim(DelimToken::LeftBrace) => self.parse_block()?,
            _ => self.parse_stmt()?,
        };
        Ok(statement)
    }

    // block
    fn parse_block(&mut self) -> ParseResult {
        let block_token = self.consume_expecting(TokenClass::Delim(DelimToken::LeftBrace))?;
        let mut declarations: Vec<Box<AstNode>> = Vec::new();
        while !matches!(
            self.peek()?.token_class,
            TokenClass::Delim(DelimToken::RightBrace | DelimToken::EoF)
        ) {
            match self.parse_decl() {
                Ok(decl) => declarations.push(decl),
                Err(_) => {
                    declarations.push(Box::new(AstNode::Error));
                    self.recover(TokenClass::Delim(DelimToken::Semicolon));
                }
            }
        }
        self.consume_expecting(TokenClass::Delim(DelimToken::RightBrace))?;
        Ok(Box::new(AstNode::Block {
            token: block_token,
            declarations,
        }))
    }

    // declarations
    fn parse_variable_decl(&mut self) -> ParseResult {
        self.consume_expecting(TokenClass::Keyword(KeywordToken::Let))?;
        let token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;
        self.consume_expecting(TokenClass::Op(OpToken::Eq))?;
        let result = Ok(Box::new(AstNode::VariableDecl {
            identifier: token.lexeme.clone(),
            token,
            expression: self.parse_expr(0.0)?,
        }));
        self.consume_expecting(TokenClass::Delim(DelimToken::Semicolon))?;
        result
    }

    fn parse_class_decl(&mut self) -> ParseResult {
        todo!();
        self.consume_expecting(TokenClass::Keyword(KeywordToken::Class))?;
        let identifier_token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;
        self.consume_expecting(TokenClass::Delim(DelimToken::LeftBrace))?;
        let mut methods: Vec<Box<AstNode>> = Vec::new();
        while self.peek()?.token_class != TokenClass::Delim(DelimToken::RightBrace) {
            methods.push(self.parse_decl()?);
        }
        self.consume_expecting(TokenClass::Delim(DelimToken::RightBrace))?;
        Ok(Box::new(AstNode::ClassDecl {
            token: identifier_token.clone(),
            name: identifier_token.lexeme.clone(),
            fields: Vec::new(),                    // TODO: parse fields
            constructor: Box::new(AstNode::Empty), // TODO: parse constructor
            methods,
        }))
    }

    fn parse_function_decl(&mut self) -> ParseResult {
        self.consume_expecting(TokenClass::Keyword(KeywordToken::Fn));
        let fn_name_token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;

        // parameters
        self.consume_expecting(TokenClass::Delim(DelimToken::LeftParen))?;
        let mut parameters: Vec<Parameter> = Vec::new();
        while self.peek()?.token_class != TokenClass::Delim(DelimToken::RightParen) {
            let param_token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;
            parameters.push(Parameter {
                token: param_token.clone(),
                name: param_token.lexeme.clone(),
            });

            if self.peek()?.token_class == TokenClass::Delim(DelimToken::Comma) {
                self.consume_expecting(TokenClass::Delim(DelimToken::Comma))?;
            }
        }
        self.consume_expecting(TokenClass::Delim(DelimToken::RightParen))?;

        // function body
        let body = self.parse_block()?;

        Ok(Box::new(AstNode::FnDecl {
            token: fn_name_token.clone(),
            name: fn_name_token.lexeme.clone(),
            parameters,
            body,
        }))
    }

    // statements
    fn parse_stmt(&mut self) -> ParseResult {
        let token = self.peek()?;
        let result = match &token.token_class {
            TokenClass::Keyword(keyword) => match keyword {
                KeywordToken::Print => self.parse_print_stmt(),
                KeywordToken::Return => self.parse_return_stmt(),
                KeywordToken::If => self.parse_if_stmt(),
                KeywordToken::While => self.parse_while_stmt(),
                KeywordToken::For => self.parse_for_stmt(),
                _ => todo!(),
            },
            TokenClass::Atom(AtomToken::Identifier)
                if matches!(
                    self.lookahead(1)?.token_class,
                    TokenClass::Op(
                        OpToken::PlusEq
                            | OpToken::MinEq
                            | OpToken::SlashEq
                            | OpToken::StarEq
                            | OpToken::Eq,
                    )
                ) =>
            {
                self.parse_assignment_stmt()
            }

            _ => self.parse_expr_stmt(),
        };
        result
    }

    fn parse_expr_stmt(&mut self) -> ParseResult {
        let expr = self.parse_expr(0.0)?;
        let token = self.peek()?;
        let result = Ok(Box::new(AstNode::ExprStmt {
            token: token.clone(),
            expression: expr,
        }));
        debug!("parsed expr stmt");
        self.consume_expecting(TokenClass::Delim(DelimToken::Semicolon))?;
        result
    }

    fn parse_assignment_stmt(&mut self) -> ParseResult {
        let identifier_token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;
        let reassignment_token = self.consume()?;
        if !matches!(
            reassignment_token.token_class,
            TokenClass::Op(
                OpToken::PlusEq | OpToken::MinEq | OpToken::SlashEq | OpToken::StarEq | OpToken::Eq
            )
        ) {
            self.errors.push(ParserError::ExpectedReassignmentOperator {
                token: reassignment_token,
            });
            return Err(());
        }

        let result = Ok(Box::new(AstNode::AssignmentDecl {
            identifier: identifier_token.lexeme.clone(),
            token: reassignment_token,
            expression: self.parse_expr(0.0)?,
        }));
        self.consume_expecting(TokenClass::Delim(DelimToken::Semicolon))?;
        result
    }

    fn parse_print_stmt(&mut self) -> ParseResult {
        let print_token = self.consume_expecting(TokenClass::Keyword(KeywordToken::Print))?;
        let result = Ok(Box::new(AstNode::PrintStmt {
            token: print_token,
            expression: self.parse_expr(0.0)?,
        }));
        self.consume_expecting(TokenClass::Delim(DelimToken::Semicolon))?;
        result
    }

    fn parse_return_stmt(&mut self) -> ParseResult {
        let return_token = self.consume_expecting(TokenClass::Keyword(KeywordToken::Return))?;
        let result = Ok(Box::new(AstNode::ReturnStmt {
            token: return_token,
            expression: self.parse_expr(0.0)?,
        }));
        self.consume_expecting(TokenClass::Delim(DelimToken::Semicolon))?;
        result
    }

    fn parse_if_stmt(&mut self) -> ParseResult {
        let if_token = self.consume_expecting(TokenClass::Keyword(KeywordToken::If))?;

        // then branch
        self.consume_expecting(TokenClass::Delim(DelimToken::LeftParen))?;
        let condition = self.parse_expr(0.0)?;
        self.consume_expecting(TokenClass::Delim(DelimToken::RightParen))?;
        let then_branch = self.parse_block()?;
        // else branch
        let else_branch: Option<Box<AstNode>> = match self.peek()?.token_class {
            TokenClass::Keyword(KeywordToken::Else) => {
                self.consume_expecting(TokenClass::Keyword(KeywordToken::Else))?;
                Some(self.parse_block()?)
            }
            _ => None,
        };

        Ok(Box::new(AstNode::IfStmt {
            token: if_token,
            condition,
            then_branch,
            else_branch,
        }))
    }

    fn parse_while_stmt(&mut self) -> ParseResult {
        let while_token = self.consume_expecting(TokenClass::Keyword(KeywordToken::While))?;
        self.consume_expecting(TokenClass::Delim(DelimToken::LeftParen))?;
        let condition = self.parse_expr(0.0)?;
        self.consume_expecting(TokenClass::Delim(DelimToken::RightParen))?;
        let body = self.parse_block()?;
        Ok(Box::new(AstNode::WhileStmt {
            token: while_token,
            condition,
            body,
        }))
    }

    fn parse_for_stmt(&mut self) -> ParseResult {
        todo!();
    }

    // pratt-parsing binary operators
    fn parse_expr(&mut self, min_bp: f32) -> ParseResult {
        let mut lhs = self.parse_null_denotation()?;

        loop {
            let op_token = self.peek()?;
            let infix_op = match &op_token.token_class {
                TokenClass::Delim(delim)
                    if matches!(
                        delim,
                        DelimToken::EoF | DelimToken::Semicolon | DelimToken::RightParen
                    ) =>
                {
                    break;
                }
                TokenClass::Op(op) => op,
                _ => {
                    self.errors
                        .push(ParserError::ExpectedOpToken { token: op_token });
                    return Err(());
                }
            };

            let (l_bp, r_bp) = match infix_bp(&infix_op) {
                Some(bp) => bp,
                None => break,
            };
            if l_bp < min_bp {
                break;
            }

            let consumed_op_token = self.consume()?;

            let rhs = self.parse_expr(r_bp)?;
            lhs = Box::new(AstNode::BinaryExpr {
                token: consumed_op_token,
                left: lhs,
                right: rhs,
            })
        }

        Ok(lhs)
    }

    fn parse_null_denotation(&mut self) -> ParseResult {
        let token = self.peek()?;
        match token.token_class {
            TokenClass::Atom(ref atom) => match atom {
                AtomToken::NumericLit => self.parse_numeric_lit(),
                AtomToken::StringLit => self.parse_string_lit(),
                AtomToken::Identifier => self.parse_identifier(),
            },
            TokenClass::Op(ref op) => match op {
                OpToken::Min | OpToken::Bang => self.parse_unary_expr(),
                _ => {
                    self.errors
                        .push(ParserError::UnexpectedUnaryOperator { token });
                    return Err(());
                }
            },
            TokenClass::Keyword(ref keyword) => match keyword {
                KeywordToken::True | KeywordToken::False => self.parse_boolean_lit(),
                KeywordToken::Nil => Ok(Box::new(AstNode::NilLit { token })),
                _ => {
                    self.errors.push(ParserError::UnexpectedKeyword { token });
                    Err(())
                }
            },
            TokenClass::Delim(DelimToken::LeftParen) => self.parse_nested_expr(),
            _ => Ok(Box::new(AstNode::NilLit { token })),
        }
    }

    fn parse_numeric_lit(&mut self) -> ParseResult {
        let token = self.consume_expecting(TokenClass::Atom(AtomToken::NumericLit))?;
        Ok(Box::new(AstNode::NumericLit {
            value: token.lexeme.parse().unwrap(),
            token: token,
        }))
    }

    fn parse_string_lit(&mut self) -> ParseResult {
        let token = self.consume_expecting(TokenClass::Atom(AtomToken::StringLit))?;
        Ok(Box::new(AstNode::StringLit {
            value: token.lexeme.clone(),
            token: token,
        }))
    }

    fn parse_boolean_lit(&mut self) -> ParseResult {
        let token = self.consume()?;
        let value = match token.token_class {
            TokenClass::Keyword(KeywordToken::True) => true,
            TokenClass::Keyword(KeywordToken::False) => false,
            _ => {
                self.errors.push(ParserError::UnexpectedKeyword {
                    token: token.clone(),
                });
                return Err(());
            }
        };
        Ok(Box::new(AstNode::BooleanLit { token, value }))
    }

    fn parse_identifier(&mut self) -> ParseResult {
        let token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;
        Ok(Box::new(AstNode::IdentifierAccessExpr {
            identifier: token.lexeme.parse().unwrap(),
            token: token,
        }))
    }

    fn parse_unary_expr(&mut self) -> ParseResult {
        let token = self.peek()?;
        let op = match token.token_class {
            TokenClass::Op(op) => op,
            _ => {
                self.errors.push(ParserError::ExpectedOpToken { token });
                return Err(());
            }
        };

        let (_, right_bp) = infix_bp(&op).unwrap();
        let operand = self.parse_expr(right_bp)?;
        Ok(Box::new(AstNode::UnaryExpr {
            token: self.consume()?,
            operand,
        }))
    }

    fn parse_nested_expr(&mut self) -> ParseResult {
        self.consume_expecting(TokenClass::Delim(DelimToken::LeftParen))?;
        let nested_expression = self.parse_expr(0.0)?;
        match self.consume_expecting(TokenClass::Delim(DelimToken::RightParen)) {
            Err(_) => {
                let token = self.peek()?;
                self.errors.push(ParserError::UnclosedExpression { token });
                Err(())
            }
            Ok(_) => Ok(nested_expression),
        }
    }
}
