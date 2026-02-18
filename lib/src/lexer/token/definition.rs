use crate::lexer::token::types::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate) r#type: TokenType,
    pub(crate) unparsed: String,
    pub(crate) line: usize,
}
