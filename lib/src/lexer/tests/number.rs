#[cfg(test)]
mod test {
    use crate::lexer::{
        tests::definition::LexerTest,
        token::{
            Token,
            types::{LiteralToken, TokenType},
        },
    };

    #[test]
    pub fn int_single() {
        LexerTest::expect_single_token("1", TokenType::Literal(LiteralToken::Integer(1)));
    }

    #[test]
    pub fn int_single2() {
        LexerTest::expect_single_token("10", TokenType::Literal(LiteralToken::Integer(10)));
    }

    #[test]
    pub fn int_single3() {
        LexerTest::expect_single_token("123", TokenType::Literal(LiteralToken::Integer(123)));
    }

    #[test]
    pub fn float_single() {
        LexerTest::expect_single_token("1.0", TokenType::Literal(LiteralToken::Float(1.0)));
    }

    #[test]
    pub fn float_malformed() {
        LexerTest::single_line_test(
            "1.",
            vec![Token::new(
                TokenType::Literal(LiteralToken::Integer(1)),
                "1.".to_string(),
                1,
                1,
            )],
        );
    }

    #[test]
    pub fn multiple_floats() {
        let input = r#"1.0 2.0"#;

        LexerTest::single_line_test(
            input,
            vec![
                Token::new(TokenType::Literal(LiteralToken::Float(1.0)), "1.0", 1, 1),
                Token::new(TokenType::Literal(LiteralToken::Float(2.0)), "2.0", 1, 3),
            ],
        );
    }
}
