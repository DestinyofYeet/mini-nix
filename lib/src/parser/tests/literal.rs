#[cfg(test)]
mod test {

    use crate::parser::{
        error::ParserError,
        parse_text,
        tests::definition::ParserTest,
        token::{
            Token,
            types::{LiteralToken, TokenType},
        },
    };

    #[test]
    pub fn identifier_single() {
        ParserTest::expect_single_token(
            "a",
            TokenType::Literal(LiteralToken::Identifier("a".to_string())),
        );
    }

    #[test]
    pub fn identifier_multi() {
        ParserTest::single_line_test(
            "a a",
            vec![
                Token::new(
                    TokenType::Literal(LiteralToken::Identifier("a".to_string())),
                    "a".to_string(),
                    1,
                ),
                Token::new(
                    TokenType::Literal(LiteralToken::Identifier("a".to_string())),
                    "a".to_string(),
                    1,
                ),
            ],
        );
    }

    #[test]
    pub fn string_single() {
        ParserTest::expect_single_token(
            r#""a""#,
            TokenType::Literal(LiteralToken::String("a".to_string())),
        );
    }

    #[test]
    pub fn string_multi() {
        ParserTest::expect_single_token(
            r#""abc""#,
            TokenType::Literal(LiteralToken::String("abc".to_string())),
        );
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
