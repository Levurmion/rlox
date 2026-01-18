#[cfg(test)]
mod tests {
    use crate::lexer::{
        lexer::LexerError,
        tokens::{AtomToken, DelimToken, KeywordToken, OpToken, TokenClass},
    };

    #[test]
    fn test_arithmetic_expr() {
        let input = String::from("2 + 2;");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        lexer.tokenize().unwrap();
        let tokens = lexer.into_token_types();

        assert_eq!(
            tokens,
            vec![
                TokenClass::Atom(AtomToken::NumericLit),
                TokenClass::Op(OpToken::Plus),
                TokenClass::Atom(AtomToken::NumericLit),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Delim(DelimToken::EoF)
            ]
        );
    }

    #[test]
    fn test_boolean_expr() {
        let input = String::from("true && false;");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        lexer.tokenize().unwrap();
        let tokens = lexer.into_token_types();

        assert_eq!(
            tokens,
            vec![
                TokenClass::Keyword(crate::lexer::tokens::KeywordToken::True),
                TokenClass::Op(OpToken::And),
                TokenClass::Keyword(crate::lexer::tokens::KeywordToken::False),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Delim(DelimToken::EoF)
            ]
        );
    }

    #[test]
    fn test_identifier() {
        let input = String::from("let x = 10;");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        lexer.tokenize().unwrap();
        let tokens = lexer.into_token_types();

        assert_eq!(
            tokens,
            vec![
                TokenClass::Keyword(KeywordToken::Let),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Eq),
                TokenClass::Atom(AtomToken::NumericLit),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Delim(DelimToken::EoF)
            ]
        );
    }

    #[test]
    fn test_string_literal() {
        let input = String::from("let greeting = \"Hello, World!\";");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        lexer.tokenize().unwrap();
        let tokens = lexer.into_token_types();

        assert_eq!(
            tokens,
            vec![
                TokenClass::Keyword(KeywordToken::Let),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Eq),
                TokenClass::Atom(AtomToken::StringLit),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Delim(DelimToken::EoF)
            ]
        );
    }

    #[test]
    fn test_variable_access_expression() {
        let input = String::from("x;");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        lexer.tokenize().unwrap();
        let tokens = lexer.into_token_types();

        assert_eq!(
            tokens,
            vec![
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Delim(DelimToken::EoF)
            ]
        );
    }

    #[test]
    fn test_invalid_identifier() {
        let input = String::from("let 123abc! = 10;");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        let result = lexer.tokenize();
        assert!(result.is_err());
    }

    #[test]
    fn test_operators() {
        let input = String::from("a == b != c <= d >= e < f > g && h || i;");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        lexer.tokenize().unwrap();
        let tokens = lexer.into_token_types();

        assert_eq!(
            tokens,
            vec![
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::EqEq),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::BangEq),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Leq),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Geq),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Lt),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Gt),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::And),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Or),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Delim(DelimToken::EoF)
            ]
        );
    }

    #[test]
    fn test_braces() {
        let input = String::from("let x = { let y = 10; y };");
        let mut lexer = crate::lexer::lexer::Lexer::new(input);
        lexer.tokenize().unwrap();
        let tokens = lexer.into_token_types();

        assert_eq!(
            tokens,
            vec![
                TokenClass::Keyword(KeywordToken::Let),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Eq),
                TokenClass::Op(OpToken::LeftBrace),
                TokenClass::Keyword(KeywordToken::Let),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Op(OpToken::Eq),
                TokenClass::Atom(AtomToken::NumericLit),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Atom(AtomToken::Identifier),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Op(OpToken::RightBrace),
                TokenClass::Delim(DelimToken::Semicolon),
                TokenClass::Delim(DelimToken::EoF)
            ]
        );
    }
}
