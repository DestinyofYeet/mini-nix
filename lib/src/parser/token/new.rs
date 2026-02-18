use crate::parser::token::{Token, types::TokenType};

impl Token {
    pub fn new(r#type: TokenType, unparsed: impl ToString, line: usize) -> Self {
        Self {
            r#type,
            unparsed: unparsed.to_string(),
            line,
        }
    }
}
