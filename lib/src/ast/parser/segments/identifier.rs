use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::Literal,
    },
    lexer::token::types::{LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_identifier(&mut self) -> ParseResult {
        trace!("parse_identifier");

        let result = match self.next() {
            Some(token) => match token.r#type {
                TokenType::Literal(LiteralToken::Identifier(_)) => Literal::create(token.clone()),

                _ => {
                    self.revert();
                    return Err(self.craft_error("Expected Identifier"));
                }
            },
            None => {
                self.revert();
                return Err(self.craft_error("Expected Identifier"));
            }
        };

        Ok(result)
    }
}
