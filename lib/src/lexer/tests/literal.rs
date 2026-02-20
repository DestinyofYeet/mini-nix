#[cfg(test)]
mod test {

    use crate::lexer::{
        error::ParserError,
        parse_text,
        tests::definition::LexerTest,
        token::{
            Token,
            types::{LiteralToken, TokenType},
        },
    };

    #[test]
    pub fn identifier_single() {
        LexerTest::expect_single_token(
            "a",
            TokenType::Literal(LiteralToken::Identifier("a".to_string())),
        );
    }

    #[test]
    pub fn identifier_multi() {
        LexerTest::single_line_test(
            "a a",
            vec![
                Token::new(
                    TokenType::Literal(LiteralToken::Identifier("a".to_string())),
                    "a".to_string(),
                    1,
                    1,
                ),
                Token::new(
                    TokenType::Literal(LiteralToken::Identifier("a".to_string())),
                    "a".to_string(),
                    1,
                    3,
                ),
            ],
        );
    }

    #[test]
    pub fn string_single() {
        LexerTest::expect_single_token(
            r#""a""#,
            TokenType::Literal(LiteralToken::String("a".to_string())),
        );
    }

    #[test]
    pub fn string_multi() {
        LexerTest::expect_single_token(
            r#""abc""#,
            TokenType::Literal(LiteralToken::String("abc".to_string())),
        );
    }

    #[test]
    pub fn string_multiline() {
        let input = "\"a\nb\"".to_string();
        let result = parse_text(input).unwrap();
        let expected = vec![Token::new(
            TokenType::Literal(LiteralToken::String("a\nb".to_string())),
            "\"a\nb\"",
            2,
            0,
        )];

        assert_eq!(result, expected)
    }

    #[test]
    pub fn malformed_string_fail() {
        let input = r#"""#.to_string();

        let result = parse_text(input);

        assert!(result.is_err());

        let err = result.unwrap_err();

        assert_eq!(
            err[0],
            ParserError::UnfinishedString {
                line: (1),
                column: 1
            }
        )
    }
}
