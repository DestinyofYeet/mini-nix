use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::Unary,
    },
    lexer::token::types::{LogicToken, MathToken, TokenType},
};

impl AstParser {
    pub fn parse_unary(&mut self) -> ParseResult {
        trace!("parse_unary");
        let err = {
            if let Some(token) = self.is_match(&[
                TokenType::Math(MathToken::Minus),
                TokenType::Logic(LogicToken::Not),
            ]) {
                let unary = self.parse_unary();

                match unary {
                    Ok(right) => return Ok(Unary::new(token, right)),
                    Err(errors) => {
                        // Revert for match above
                        self.revert();
                        self.craft_error("Failed to parse unary (right):", errors)
                    }
                }
            } else {
                let primary = self.parse_primary_or_identifier();

                match primary {
                    Ok(value) => return Ok(value),
                    Err(errors) => {
                        self.craft_error("Failed to parse primary_or_identifier:", errors)
                    }
                }
            }
        };

        Err(self.craft_error("Failed to parse unary", vec![err]).into())
    }
}
