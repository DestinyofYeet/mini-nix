use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult},
        types::Binary,
    },
    lexer::token::types::{MathToken, TokenType},
};

impl AstParser {
    pub fn parse_arithmetic(&mut self) -> ParseResult {
        trace!("parse_arithmetic");

        // early return is ok here, since a arithmetic_mul is required
        let left = self.parse_arithmetic_mul()?;

        let mut return_expr = left;

        loop {
            let pre_index = self.index;

            let maybe_op = self.is_match(&[
                TokenType::Math(MathToken::Minus),
                TokenType::Math(MathToken::Plus),
            ]);

            let op = match maybe_op {
                Some(value) => value,
                None => return Ok(return_expr),
            };

            let right = match self.parse_arithmetic_mul() {
                Ok(value) => value,
                Err(errors) => {
                    // the operator matched, so a arithemetic mul has to match
                    self.index = pre_index;
                    return Err(self
                        .craft_error("Failed to parse right on arithmetic:", errors)
                        .into());
                }
            };

            return_expr = Binary::new(return_expr, op, right);
        }
    }
}
