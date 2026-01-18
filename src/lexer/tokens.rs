#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelimToken {
    Semicolon,
    EoF,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpToken {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // arithmetic
    Plus,
    Min,
    Slash,
    Star,

    // boolean
    Bang,
    EqEq,
    BangEq,
    Gt,
    Lt,
    Geq,
    Leq,
    And,
    Or,

    // assignment
    PlusEq,
    MinEq,
    SlashEq,
    StarEq,
    Eq,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomToken {
    NumericLit,
    StringLit,
    Identifier,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordToken {
    // declarations
    Let,
    Fn,
    Class,

    // statements
    Print,
    Return,
    If,
    Else,
    For,
    While,

    // bool literals
    True,
    False,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenClass {
    Delim(DelimToken),
    Op(OpToken),
    Atom(AtomToken),
    Keyword(KeywordToken),
}

#[derive(Debug, Clone)]
pub struct TokenMeta {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_class: TokenClass,
    pub lexeme: String,
    pub meta: TokenMeta,
}
