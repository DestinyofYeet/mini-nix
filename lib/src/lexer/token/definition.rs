use crate::lexer::token::types::TokenType;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Token {
    pub r#type: TokenType,
    pub unparsed: String,
    pub line: usize,
    pub column: usize,
}
