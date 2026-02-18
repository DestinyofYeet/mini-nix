#[cfg(test)]
mod test {
    use crate::parser::{
        parse_text,
        tests::definition::ParserTest,
        token::{
            Token,
            types::{LiteralToken, MathToken, TokenType},
        },
    };

    #[test]
    pub fn addition_int() {
        let input = r#"1 + 2"#;

        ParserTest::single_line_test(
            input,
            vec![
                Token::new(TokenType::Literal(LiteralToken::Integer(1)), "1", 1),
                Token::new(TokenType::Math(MathToken::Plus), "+", 1),
                Token::new(TokenType::Literal(LiteralToken::Integer(2)), "2", 1),
            ],
        );
    }

    #[test]
    pub fn addition_float() {
        let input = r#"1.0 + 2.0"#;

        ParserTest::single_line_test(
            input,
            vec![
                Token::new(TokenType::Literal(LiteralToken::Float(1.0)), "1.0", 1),
                Token::new(TokenType::Math(MathToken::Plus), "+", 1),
                Token::new(TokenType::Literal(LiteralToken::Float(2.0)), "2.0", 1),
            ],
        );
    }
}
