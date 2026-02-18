#[cfg(test)]
mod test {
    use crate::lexer::{
        parse_text,
        tests::definition::LexerTest,
        token::{
            Token,
            types::{LiteralToken, MathToken, TokenType},
        },
    };

    #[test]
    pub fn addition_int() {
        let input = r#"1 + 2"#;

        LexerTest::single_line_test(
            input,
            vec![
                Token::new(TokenType::Literal(LiteralToken::Integer(1)), "1", 1, 1),
                Token::new(TokenType::Math(MathToken::Plus), "+", 1, 3),
                Token::new(TokenType::Literal(LiteralToken::Integer(2)), "2", 1, 5),
            ],
        );
    }

    #[test]
    pub fn addition_float() {
        let input = r#"1.0 + 2.0"#;

        LexerTest::single_line_test(
            input,
            vec![
                Token::new(TokenType::Literal(LiteralToken::Float(1.0)), "1.0", 1, 1),
                Token::new(TokenType::Math(MathToken::Plus), "+", 1, 3),
                Token::new(TokenType::Literal(LiteralToken::Float(2.0)), "2.0", 1, 5),
            ],
        );
    }
}
