use crate::parser::token::{Token, types::TokenType};

impl Token {
    pub fn new(r#type: TokenType, unparsed: String, line: usize) -> Self {
        Self {
            r#type,
            unparsed,
            line,
        }
    }
}
