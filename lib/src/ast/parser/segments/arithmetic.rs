use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::Binary,
    },
    lexer::token::types::{MathToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_arithmetic(&mut self) -> ParseResult {
        trace!("parse_arithmetic");

        let has_left_parent = self
            .is_match(&[TokenType::Misc(MiscToken::LeftParen)])
            .is_some();

        let left = match self.parse_arithmetic_or_primary() {
            Ok(value) => value,
            Err(e) => {
                if has_left_parent {
                    self.revert();
                }

                self.revert();

                return Err(e);
            }
        };

        let operator = match self.is_match(&[
            TokenType::Math(MathToken::Minus),
            TokenType::Math(MathToken::Plus),
            TokenType::Math(MathToken::Slash),
            TokenType::Math(MathToken::Star),
        ]) {
            Some(token) => token,
            None => {
                self.revert();
                return Err(self.craft_error("Expected operator"));
            }
        };

        let right = match self.parse_arithmetic_or_primary() {
            Ok(value) => value,
            Err(e) => {
                if has_left_parent {
                    self.revert();
                }

                self.revert();

                return Err(e);
            }
        };

        if has_left_parent
            && self
                .is_match(&[TokenType::Misc(MiscToken::RightParen)])
                .is_none()
        {
            return Err(self.craft_error("Expected ')'"));
        }

        trace!(
            "expr: left: {:?}, operator: {:?}, right: {:?}",
            left, operator, right
        );

        let final_expr = Binary::create(left, operator, right);

        Ok(final_expr)
    }
}
