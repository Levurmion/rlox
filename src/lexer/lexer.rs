use crate::lexer::tokens::{
    AtomToken, DelimToken, KeywordToken, OpToken, Token, TokenClass, TokenMeta,
};

#[derive(Debug, Clone)]
pub enum LexerError {
    UnexpectedEndOfFile { meta: TokenMeta },
    UnexpectedCharacter { char: String, meta: TokenMeta },
    InvalidNumericLit { char: String, meta: TokenMeta },
    InvalidIdentifier { char: String, meta: TokenMeta },
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
    // ============== HELPER METHODS ==============
    pub fn into_token_types(&self) -> Vec<TokenClass> {
        self.tokens
            .iter()
            .map(|token| token.token_class.clone())
            .collect()
    }

    // ============== LEXER METHODS ==============
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

    fn push_token(&mut self, token_class: TokenClass, lexeme: &str) {
        self.tokens.push(Token {
            token_class,
            lexeme: lexeme.to_string(),
            meta: TokenMeta {
                row: self.row,
                col: self.col,
            },
        });
        self.advance(lexeme.len());
    }

    fn scan_delimiter(&mut self, lexeme: String) -> Result<(), LexerError> {
        match lexeme.as_str() {
            ";" => self.push_token(TokenClass::Delim(DelimToken::Semicolon), &lexeme),
            "," => self.push_token(TokenClass::Delim(DelimToken::Comma), &lexeme),
            "(" => self.push_token(TokenClass::Delim(DelimToken::LeftParen), &lexeme),
            ")" => self.push_token(TokenClass::Delim(DelimToken::RightParen), &lexeme),
            "{" => self.push_token(TokenClass::Delim(DelimToken::LeftBrace), &lexeme),
            "}" => self.push_token(TokenClass::Delim(DelimToken::RightBrace), &lexeme),
            _ => return Err(self.create_unexpected_char_err(&lexeme)),
        }
        Ok(())
    }

    fn scan_op(&mut self, lexeme: String) -> Result<(), LexerError> {
        match lexeme.as_str() {
            "=" => match self.input.get(self.pos..self.pos + 2) {
                Some("==") => self.push_token(TokenClass::Op(OpToken::EqEq), "=="),
                _ => self.push_token(TokenClass::Op(OpToken::Eq), &lexeme),
            },
            "+" => match self.input.get(self.pos..self.pos + 2) {
                Some("+=") => self.push_token(TokenClass::Op(OpToken::PlusEq), "+="),
                _ => self.push_token(TokenClass::Op(OpToken::Plus), &lexeme),
            },
            "-" => match self.input.get(self.pos..self.pos + 2) {
                Some("-=") => self.push_token(TokenClass::Op(OpToken::MinEq), "-="),
                _ => self.push_token(TokenClass::Op(OpToken::Min), &lexeme),
            },
            "/" => match self.input.get(self.pos..self.pos + 2) {
                Some("/=") => self.push_token(TokenClass::Op(OpToken::SlashEq), "/="),
                _ => self.push_token(TokenClass::Op(OpToken::Slash), &lexeme),
            },
            "*" => match self.input.get(self.pos..self.pos + 2) {
                Some("*=") => self.push_token(TokenClass::Op(OpToken::StarEq), "*="),
                _ => self.push_token(TokenClass::Op(OpToken::Star), &lexeme),
            },
            "!" => match self.input.get(self.pos..self.pos + 2) {
                Some("!=") => self.push_token(TokenClass::Op(OpToken::BangEq), "!="),
                _ => self.push_token(TokenClass::Op(OpToken::Bang), &lexeme),
            },
            ">" => match self.input.get(self.pos..self.pos + 2) {
                Some(">=") => self.push_token(TokenClass::Op(OpToken::Geq), ">="),
                _ => self.push_token(TokenClass::Op(OpToken::Gt), &lexeme),
            },
            "<" => match self.input.get(self.pos..self.pos + 2) {
                Some("<=") => self.push_token(TokenClass::Op(OpToken::Leq), "<="),
                _ => self.push_token(TokenClass::Op(OpToken::Lt), &lexeme),
            },
            "&" if self.input.get(self.pos..self.pos + 2) == Some("&&") => {
                self.push_token(TokenClass::Op(OpToken::And), "&&");
            }
            "|" if self.input.get(self.pos..self.pos + 2) == Some("||") => {
                self.push_token(TokenClass::Op(OpToken::Or), "||");
            }
            _ => return Err(self.create_unexpected_char_err(&lexeme)),
        }
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

        let lexeme = self.input.get(self.pos..end).unwrap().to_string();
        self.push_token(TokenClass::Atom(AtomToken::NumericLit), &lexeme);

        Ok(())
    }

    fn scan_string_lit(&mut self) -> Result<(), LexerError> {
        let mut end = self.pos + 1;

        while end < self.input.len() {
            let curr = self.peek_at(end)?;
            match curr {
                "\"" => break,
                _ => end += 1,
            }
        }

        if end >= self.input.len() {
            return Err(LexerError::UnexpectedEndOfFile {
                meta: TokenMeta {
                    row: self.row,
                    col: self.col,
                },
            });
        }

        // include the closing quote
        end += 1;

        let lexeme = self.input.get(self.pos..end).unwrap().to_string();
        self.push_token(TokenClass::Atom(AtomToken::StringLit), &lexeme);

        Ok(())
    }

    fn scan_keyword(&mut self) -> Result<(), LexerError> {
        let lexeme = self.peek()?;
        match lexeme {
            "l" if self.input.get(self.pos..self.pos + 3) == Some("let") => {
                self.push_token(TokenClass::Keyword(KeywordToken::Let), "let");
            }
            "p" if self.input.get(self.pos..self.pos + 5) == Some("print") => {
                self.push_token(TokenClass::Keyword(KeywordToken::Print), "print");
            }
            "f" if self.input.get(self.pos..self.pos + 2) == Some("fn") => {
                self.push_token(TokenClass::Keyword(KeywordToken::Fn), "fn");
            }
            "f" if self.input.get(self.pos..self.pos + 5) == Some("false") => {
                self.push_token(TokenClass::Keyword(KeywordToken::False), "false");
            }
            "f" if self.input.get(self.pos..self.pos + 3) == Some("for") => {
                self.push_token(TokenClass::Keyword(KeywordToken::For), "for");
            }
            "r" if self.input.get(self.pos..self.pos + 6) == Some("return") => {
                self.push_token(TokenClass::Keyword(KeywordToken::Return), "return");
            }
            "i" if self.input.get(self.pos..self.pos + 2) == Some("if") => {
                self.push_token(TokenClass::Keyword(KeywordToken::If), "if");
            }
            "e" if self.input.get(self.pos..self.pos + 4) == Some("else") => {
                self.push_token(TokenClass::Keyword(KeywordToken::Else), "else");
            }
            "w" if self.input.get(self.pos..self.pos + 5) == Some("while") => {
                self.push_token(TokenClass::Keyword(KeywordToken::While), "while");
            }
            "t" if self.input.get(self.pos..self.pos + 4) == Some("true") => {
                self.push_token(TokenClass::Keyword(KeywordToken::True), "true");
            }
            "c" if self.input.get(self.pos..self.pos + 5) == Some("class") => {
                self.push_token(TokenClass::Keyword(KeywordToken::Class), "class");
            }
            "n" if self.input.get(self.pos..self.pos + 3) == Some("nil") => {
                self.push_token(TokenClass::Keyword(KeywordToken::Nil), "nil");
            }
            _ => self.scan_identifier()?,
        }

        Ok(())
    }

    fn scan_identifier(&mut self) -> Result<(), LexerError> {
        let mut end = self.pos + 1;

        while end < self.input.len() {
            let curr = self.peek_at(end)?;
            match curr {
                " " | ";" => break,
                _ => end += 1,
            }
        }

        let lexeme = self.input.get(self.pos..end).unwrap().to_string();
        if lexeme
            .chars()
            .all(|char| char.is_alphanumeric() || char == '_')
        {
            self.push_token(TokenClass::Atom(AtomToken::Identifier), &lexeme);
            Ok(())
        } else {
            Err(LexerError::InvalidIdentifier {
                char: lexeme,
                meta: TokenMeta {
                    row: self.row,
                    col: self.col,
                },
            })
        }
    }

    pub fn tokenize(&mut self) -> Result<(), LexerError> {
        while self.pos < self.input.len() {
            let lexeme = self.peek()?;
            match lexeme {
                " " => self.advance(1),
                "\n" => self.new_line(),
                ";" | "," | "(" | ")" | "{" | "}" => self.scan_delimiter(lexeme.to_string())?,
                "=" | "+" | "-" | "/" | "*" | "!" | ">" | "<" | "&" | "|" => {
                    self.scan_op(lexeme.to_string())?
                }
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.scan_num_lit()?,
                "\"" => self.scan_string_lit()?,
                _ => self.scan_keyword()?,
            }
        }

        self.push_token(TokenClass::Delim(DelimToken::EoF), "");

        Ok(())
    }
}
