use crate::lexer::{
    parse_text,
    token::{Token, types::TokenType},
};

pub(super) struct LexerTest {}
impl LexerTest {
    /// Do a test.
    pub(super) fn single_line_test(input: &str, expected: Vec<Token>) {
        assert_eq!(parse_text(input.to_string()).unwrap(), expected);
    }

    pub(super) fn expect_single_token(input: &str, token: TokenType) {
        let expected = vec![Token::new(token, input.to_string(), 1, 1)];

        Self::single_line_test(input, expected);
    }
}
