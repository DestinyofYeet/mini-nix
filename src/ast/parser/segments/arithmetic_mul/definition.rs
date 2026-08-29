use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::Binary,
    },
    lexer::token::types::{MathToken, TokenType},
};

impl AstParser {
    pub fn parse_arithmetic_mul(&mut self) -> ParseResult {
        trace!("parse_arithmetic_mul");

        // early return is ok here, since a unary is required
        let left = self.parse_unary()?;

        let mut return_expr = left;

        loop {
            let pre_index = self.index;

            let maybe_op = self.is_match(&[
                TokenType::Math(MathToken::Slash),
                TokenType::Math(MathToken::Star),
            ]);

            let op = match maybe_op {
                Some(value) => value,
                None => return Ok(return_expr),
            };

            let right = match self.parse_unary() {
                Ok(value) => value,
                Err(errors) => {
                    // the operator matched, but the unary didn't. Need to reset this operation entirely.
                    self.index = pre_index;
                    return Err(self
                        .craft_error("Failed to parse right on arithmetic_mul:", errors)
                        .into());
                }
            };

            return_expr = Binary::new(return_expr, op, right);
        }
    }
}
