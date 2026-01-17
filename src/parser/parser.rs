use crate::{
    lexer::tokens::{AtomToken, DelimToken, KeywordToken, OpToken, Token, TokenClass},
    parser::ast::{AstNode, ParserError},
};

pub type ParseResult = Result<Box<AstNode>, ParserError>;

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
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser { tokens, pos: 0 }
    }

    // utilities
    fn peek(&self) -> Result<&Token, ParserError> {
        match self.tokens.get(self.pos) {
            None => Err(ParserError::UnexpectedEndOfTokenStream),
            Some(token) => Ok(token),
        }
    }

    fn consume(&mut self) -> Result<Token, ParserError> {
        let consumed = self.peek()?.clone();
        self.pos += 1;
        Ok(consumed)
    }

    fn consume_expecting(&mut self, expected: TokenClass) -> Result<Token, ParserError> {
        let consumed = self.consume()?;
        if consumed.token_class == expected {
            return Ok(consumed);
        }
        Err(ParserError::UnexpectedToken {
            token: consumed,
            expected: Some(expected),
        })
    }

    pub fn recover(&mut self, to_token: TokenClass) {
        loop {
            match self.peek() {
                Ok(token) if token.token_class == to_token => break,
                Err(_) => break,
                _ => self.pos += 1,
            }
        }
    }

    // parsing
    pub fn parse(&mut self) -> ParseResult {
        let program = self.parse_tokens()?;
        match self.consume() {
            Ok(token) if token.token_class == TokenClass::Delim(DelimToken::EoF) => Ok(program),
            Ok(token) => Err(ParserError::ExpectedEoF { token }),
            Err(err) => Err(err),
        }
    }

    fn parse_tokens(&mut self) -> ParseResult {
        let token = self.peek()?;
        match token.token_class {
            TokenClass::Atom(_) | TokenClass::Op(_) => self.parse_expr(0.0),
            _ => self.parse_stmt(),
        }
    }

    // statements
    fn parse_stmt(&mut self) -> ParseResult {
        let token = self.peek()?;
        let statement = match &token.token_class {
            TokenClass::Keyword(keyword) => match keyword {
                KeywordToken::Let => self.parse_variable_declaration_stmt()?,
                KeywordToken::Print => self.parse_print_stmt()?,
                _ => todo!(),
            },
            TokenClass::Atom(AtomToken::Identifier) => self.parse_variable_reassignment_stmt()?,
            _ => todo!(),
        };
        self.consume_expecting(TokenClass::Delim(DelimToken::Semicolon))?;
        Ok(statement)
    }

    fn parse_variable_declaration_stmt(&mut self) -> ParseResult {
        self.consume_expecting(TokenClass::Keyword(KeywordToken::Let))?;
        let token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;
        self.consume_expecting(TokenClass::Op(OpToken::Eq))?;
        Ok(Box::new(AstNode::VariableDecl {
            identifier: token.lexeme.clone(),
            token,
            expression: self.parse_expr(0.0)?,
        }))
    }

    fn parse_variable_reassignment_stmt(&mut self) -> ParseResult {
        let identifier_token = self.consume_expecting(TokenClass::Atom(AtomToken::Identifier))?;
        let reassignment_token = self.consume()?;
        if !matches!(
            reassignment_token.token_class,
            TokenClass::Op(OpToken::PlusEq | OpToken::MinEq | OpToken::SlashEq | OpToken::StarEq)
        ) {
            return Err(ParserError::ExpectedReassignmentOperator {
                token: reassignment_token,
            });
        }

        Ok(Box::new(AstNode::VariableReassignDecl {
            identifier: identifier_token.lexeme.clone(),
            token: reassignment_token,
            expression: self.parse_expr(0.0)?,
        }))
    }

    fn parse_print_stmt(&mut self) -> ParseResult {
        let print_token = self.consume_expecting(TokenClass::Keyword(KeywordToken::Print))?;
        Ok(Box::new(AstNode::PrintStmt {
            token: print_token,
            expression: self.parse_expr(0.0)?,
        }))
    }

    // pratt-parsing binary operators
    fn parse_expr(&mut self, min_bp: f32) -> ParseResult {
        let lhs_token = self.consume()?;
        let mut lhs = self.parse_null_denotation(&lhs_token)?;

        loop {
            let op_token = self.peek()?;
            let infix_op = match &op_token.token_class {
                TokenClass::Delim(delim)
                    if matches!(delim, DelimToken::EoF | DelimToken::Semicolon) =>
                {
                    break;
                }
                TokenClass::Op(op) => op,
                _ => {
                    return Err(ParserError::ExpectedOpToken {
                        token: op_token.clone(),
                    });
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

    fn parse_null_denotation(&mut self, token: &Token) -> ParseResult {
        match token.token_class {
            TokenClass::Atom(ref atom) => match atom {
                AtomToken::NumericLit => self.parse_numeric_lit(token),
                AtomToken::StringLit => self.parse_string_lit(token),
                AtomToken::Identifier => self.parse_identifier(token),
            },
            TokenClass::Op(ref op) => match op {
                OpToken::Min | OpToken::Bang => self.parse_unary_expr(token),
                OpToken::LeftParen => self.parse_nested_expr(token),
                _ => {
                    return Err(ParserError::UnexpectedUnaryOperator {
                        token: token.clone(),
                    });
                }
            },
            TokenClass::Keyword(ref keyword) => match keyword {
                KeywordToken::True | KeywordToken::False => self.parse_boolean_lit(token),
                _ => {
                    return Err(ParserError::UnexpectedKeyword {
                        token: token.clone(),
                    });
                }
            },
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: token.clone(),
                    expected: None,
                });
            }
        }
    }

    fn parse_numeric_lit(&self, token: &Token) -> ParseResult {
        Ok(Box::new(AstNode::NumericLit {
            token: token.clone(),
            value: token.lexeme.parse().unwrap(),
        }))
    }

    fn parse_string_lit(&self, token: &Token) -> ParseResult {
        Ok(Box::new(AstNode::StringLit {
            token: token.clone(),
            value: token.lexeme.clone(),
        }))
    }

    fn parse_boolean_lit(&self, token: &Token) -> ParseResult {
        let value = match token.token_class {
            TokenClass::Keyword(KeywordToken::True) => true,
            TokenClass::Keyword(KeywordToken::False) => false,
            _ => {
                return Err(ParserError::UnexpectedKeyword {
                    token: token.clone(),
                });
            }
        };
        Ok(Box::new(AstNode::BooleanLit {
            token: token.clone(),
            value,
        }))
    }

    fn parse_identifier(&self, token: &Token) -> ParseResult {
        Ok(Box::new(AstNode::VariableAccessExpr {
            token: token.clone(),
            identifier: token.lexeme.parse().unwrap(),
        }))
    }

    fn parse_unary_expr(&mut self, token: &Token) -> ParseResult {
        let op = match &token.token_class {
            TokenClass::Op(op) => op,
            _ => {
                return Err(ParserError::ExpectedOpToken {
                    token: token.clone(),
                });
            }
        };

        let (_, right_bp) = infix_bp(op).unwrap();
        let operand = self.parse_expr(right_bp)?;
        Ok(Box::new(AstNode::UnaryExpr {
            token: token.clone(),
            operand,
        }))
    }

    fn parse_nested_expr(&mut self, token: &Token) -> ParseResult {
        let nested_expression = self.parse_expr(0.0)?;
        let expression_end_token = self.consume()?;
        if expression_end_token.token_class == TokenClass::Op(OpToken::RightParen) {
            Ok(Box::new(AstNode::Expr {
                token: token.clone(),
                expr: nested_expression,
            }))
        } else {
            Err(ParserError::UnclosedExpression {
                token: token.clone(),
            })
        }
    }
}
