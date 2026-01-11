use std::{cell::RefCell, rc::Rc};

use crate::{
    lexer::{AtomToken, DelimToken, OpToken, Token, TokenClass},
    parser::ast::{AstNode, ParserError},
};

macro_rules! return_unwrapped_error {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return e,
        }
    };
}

type AstNodeRef = Rc<RefCell<AstNode>>;

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
    pub ast: AstNodeRef,
    pub errors: Vec<AstNodeRef>,
    tokens: &'a Vec<Token>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            ast: Rc::new(RefCell::new(AstNode::Empty)),
            errors: Vec::new(),
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Result<&Token, ParserError> {
        match self.tokens.get(self.pos) {
            None => Err(ParserError::UnexpectedEndOfTokenStream),
            Some(token) => Ok(token),
        }
    }

    fn peek_or_report(&mut self) -> Result<Token, AstNodeRef> {
        match self.peek() {
            Ok(token) => Ok(token.clone()),
            Err(err) => Err(self.report_error(err, None)),
        }
    }

    fn consume(&mut self) -> Result<Token, AstNodeRef> {
        let token = self.peek_or_report()?;
        self.pos += 1;
        Ok(token)
    }

    /**
    Create an `ErrorNode` with the given `error`, push to `self.errors`, and return as `AstNodeRef`
    to mark a parsing error in the `ast`.
    */
    fn report_error(&mut self, error: ParserError, token: Option<Token>) -> AstNodeRef {
        let error_node = Rc::new(RefCell::new(AstNode::ErrorNode { error, token }));
        self.errors.push(Rc::clone(&error_node));
        return Rc::clone(&error_node);
    }

    pub fn parse(&mut self) -> Result<(), usize> {
        let program = self.parse_tokens();
        match self.consume() {
            Ok(token) if token.token_class != TokenClass::Delim(DelimToken::EoF) => {
                self.report_error(ParserError::ExpectedEoF, Some(token));
            }
            Err(_) => {}
            _ => {}
        }

        self.ast = program;

        if self.errors.len() > 0 {
            return Err(self.errors.len());
        }
        return Ok(());
    }

    fn parse_tokens(&mut self) -> AstNodeRef {
        let token = return_unwrapped_error!(self.peek_or_report());
        match token.token_class {
            TokenClass::Atom(_) | TokenClass::Op(_) => self.parse_expr(0.0),
            _ => todo!(),
        }
    }

    fn parse_expr(&mut self, min_bp: f32) -> AstNodeRef {
        let lhs_token = return_unwrapped_error!(self.consume());
        let mut lhs = match lhs_token.token_class {
            TokenClass::Atom(ref atom) => match atom {
                AtomToken::NumericLit => self.parse_numeric_lit(&lhs_token),
            },
            TokenClass::Op(ref op) => match op {
                OpToken::Min => self.parse_unary_expr(&lhs_token),
                OpToken::LeftParen => self.parse_nested_expr(&lhs_token),
                _ => {
                    return self.report_error(
                        ParserError::UnexpectedUnaryOperator,
                        Some(lhs_token.clone()),
                    );
                }
            },
            _ => return self.report_error(ParserError::UnexpectedToken, Some(lhs_token)),
        };

        loop {
            let op_token = return_unwrapped_error!(self.peek_or_report());
            let infix_op = match &op_token.token_class {
                TokenClass::Op(op) => op,
                TokenClass::Delim(delim) if *delim == DelimToken::EoF => break,
                _ => return self.report_error(ParserError::ExpectedOpToken, Some(lhs_token)),
            };

            let (l_bp, r_bp) = match infix_bp(&infix_op) {
                Some(bp) => bp,
                None => break,
            };
            if l_bp < min_bp {
                break;
            }

            return_unwrapped_error!(self.consume());

            let rhs = self.parse_expr(r_bp);
            lhs = Rc::new(RefCell::new(AstNode::BinaryExpr {
                op: op_token,
                left: lhs,
                right: rhs,
            }))
        }

        lhs
    }

    fn parse_numeric_lit(&self, token: &Token) -> AstNodeRef {
        Rc::new(RefCell::new(AstNode::NumericLit {
            token: token.clone(),
            value: token.lexeme.parse().unwrap(),
        }))
    }

    fn parse_unary_expr(&mut self, token: &Token) -> AstNodeRef {
        let op = match &token.token_class {
            TokenClass::Op(op) => op,
            _ => return self.report_error(ParserError::ExpectedOpToken, Some(token.clone())),
        };

        let (_, right_bp) = infix_bp(op).unwrap();
        let operand = self.parse_expr(right_bp);
        Rc::new(RefCell::new(AstNode::UnaryExpr {
            op: token.clone(),
            operand,
        }))
    }

    fn parse_nested_expr(&mut self, token: &Token) -> AstNodeRef {
        let nested_expression = self.parse_expr(0.0);
        let expression_end_token = return_unwrapped_error!(self.consume());
        if expression_end_token.token_class == TokenClass::Op(OpToken::RightParen) {
            Rc::new(RefCell::new(AstNode::Expr {
                token: token.clone(),
                expr: nested_expression,
            }))
        } else {
            self.report_error(ParserError::UnclosedExpression, Some(token.clone()))
        }
    }
}
