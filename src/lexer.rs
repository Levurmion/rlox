#[derive(Debug)]
struct TokenMeta {
    row: usize,
    col: usize,
}

#[derive(Debug)]
pub enum TokenType {
    Semicolon,
    LeftParen,
    RightParen,
    NumericLit,
    Add,
    Sub,
    Div,
    Mul,
    EoF,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedEndOfFile { meta: TokenMeta },
    UnexpectedCharacter { char: String, meta: TokenMeta },
    InvalidNumericLit { char: String, meta: TokenMeta },
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    meta: TokenMeta,
}

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>,
    input: String,
    pos: usize,
    row: usize,
    col: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            tokens: Vec::new(),
            input,
            pos: 0,
            row: 0,
            col: 0,
        }
    }

    fn new_line(&mut self) {
        self.pos += 1;
        self.row += 1;
        self.col = 0;
    }

    fn advance(&mut self, by: usize) {
        self.pos += by;
        self.col += by;
    }

    fn peek(&self) -> Result<&str, LexerError> {
        match self.input.get(self.pos..self.pos + 1) {
            Some(top) => Ok(top),
            None => Err(LexerError::UnexpectedEndOfFile {
                meta: TokenMeta {
                    row: self.row,
                    col: self.col,
                },
            }),
        }
    }

    fn peek_at(&self, at: usize) -> Result<&str, LexerError> {
        match self.input.get(at..at + 1) {
            Some(top) => Ok(top),
            None => Err(LexerError::UnexpectedEndOfFile {
                meta: TokenMeta {
                    row: self.row,
                    col: self.col,
                },
            }),
        }
    }

    fn create_unexpected_char_err(&self, lexeme: &str) -> LexerError {
        LexerError::UnexpectedCharacter {
            char: lexeme.to_string(),
            meta: TokenMeta {
                row: self.row,
                col: self.col,
            },
        }
    }

    fn push_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            meta: TokenMeta {
                row: self.row,
                col: self.col,
            },
        });
    }

    fn scan_delimiter(&mut self, lexeme: String) -> Result<(), LexerError> {
        match lexeme.as_str() {
            ";" => self.push_token(TokenType::Semicolon, &lexeme),
            "(" => self.push_token(TokenType::LeftParen, &lexeme),
            ")" => self.push_token(TokenType::RightParen, &lexeme),
            _ => return Err(self.create_unexpected_char_err(&lexeme)),
        }
        self.advance(lexeme.len());
        Ok(())
    }

    fn scan_binary_op(&mut self, lexeme: String) -> Result<(), LexerError> {
        match lexeme.as_str() {
            "+" => self.push_token(TokenType::Add, &lexeme),
            "-" => self.push_token(TokenType::Sub, &lexeme),
            "/" => self.push_token(TokenType::Div, &lexeme),
            "*" => self.push_token(TokenType::Mul, &lexeme),
            _ => return Err(self.create_unexpected_char_err(&lexeme)),
        }
        self.advance(lexeme.len());
        Ok(())
    }

    fn scan_num_lit(&mut self) -> Result<(), LexerError> {
        let mut end = self.pos + 1;
        let mut is_float = false;

        while end < self.input.len() {
            let curr = self.peek_at(end)?;
            match curr {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => end += 1,
                "." => {
                    if is_float {
                        return Err(LexerError::InvalidNumericLit {
                            char: curr.to_string(),
                            meta: TokenMeta {
                                row: self.row,
                                col: self.col,
                            },
                        });
                    }
                    is_float = true;
                    end += 1;
                }
                _ => break,
            }
        }

        let delta = end - self.pos;
        let lexeme = self.input.get(self.pos..end).unwrap().to_string();
        self.push_token(TokenType::NumericLit, &lexeme);
        self.pos += delta;

        Ok(())
    }

    pub fn tokenize(&mut self) -> Result<(), LexerError> {
        while self.pos < self.input.len() {
            let lexeme = self.peek()?;
            match lexeme {
                " " => self.advance(1),
                "\n" => self.new_line(),
                ";" | "(" | ")" => self.scan_delimiter(lexeme.to_string())?,
                "+" | "-" | "/" | "*" => self.scan_binary_op(lexeme.to_string())?,
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.scan_num_lit()?,
                _ => return Err(self.create_unexpected_char_err(lexeme)),
            }
        }

        self.push_token(TokenType::EoF, "");

        Ok(())
    }
}
