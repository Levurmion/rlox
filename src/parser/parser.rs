use crate::{
    lexer::{AtomToken, DelimToken, OpToken, Token, TokenClass},
    parser::ast::{AstNode, ParserError},
};

pub type ParseResult = Result<Box<AstNode>, ParserError>;

fn infix_bp(op: &OpToken) -> Option<(f32, f32)> {
    match op {
        OpToken::Star => Some((10.1, 10.0)),
        OpToken::Slash => Some((9.1, 9.0)),
        OpToken::Min => Some((8.1, 11.0)),
        OpToken::Plus => Some((7.1, 7.0)),
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
            _ => todo!(),
        }
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

    fn parse_expr(&mut self, min_bp: f32) -> ParseResult {
        let lhs_token = self.consume()?;
        let mut lhs = match lhs_token.token_class {
            TokenClass::Atom(ref atom) => match atom {
                AtomToken::NumericLit => self.parse_numeric_lit(&lhs_token),
            },
            TokenClass::Op(ref op) => match op {
                OpToken::Min => self.parse_unary_expr(&lhs_token),
                OpToken::LeftParen => self.parse_nested_expr(&lhs_token),
                _ => {
                    return Err(ParserError::UnexpectedUnaryOperator {
                        token: lhs_token.clone(),
                    });
                }
            },
            _ => {
                return Err(ParserError::UnexpectedToken {
                    token: lhs_token.clone(),
                });
            }
        }?;

        loop {
            let op_token = self.peek()?;
            let infix_op = match &op_token.token_class {
                TokenClass::Op(op) => op,
                TokenClass::Delim(delim) if *delim == DelimToken::EoF => break,
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
                op: consumed_op_token,
                left: lhs,
                right: rhs,
            })
        }

        Ok(lhs)
    }

    fn parse_numeric_lit(&self, token: &Token) -> ParseResult {
        Ok(Box::new(AstNode::NumericLit {
            token: token.clone(),
            value: token.lexeme.parse().unwrap(),
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
            op: token.clone(),
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
