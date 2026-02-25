use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::SyntaxError},
        types::Binary,
    },
    lexer::token::types::{MathToken, TokenType},
};

impl AstParser {
    pub fn parse_arithmetic(&mut self) -> ParseResult {
        trace!("parse_arithmetic");

        let mut errors: Vec<SyntaxError> = Vec::new();

        let left = match self.parse_arithmetic_mul() {
            Ok(value) => value,
            Err(e) => {
                return Err(e);
            }
        };

        let mut return_expr = left;

        loop {
            let operator = match self.is_match(&[
                TokenType::Math(MathToken::Minus),
                TokenType::Math(MathToken::Plus),
            ]) {
                Some(token) => Some(token),
                None => {
                    errors.push(self.craft_error("Expected operator"));
                    None
                }
            };

            let right = match self.parse_arithmetic_mul() {
                Ok(value) => Some(value),
                Err(e) => {
                    errors.push(e);
                    None
                }
            };

            trace!(
                "expr: left: {:?}, operator: {:?}, right: {:?}",
                return_expr, operator, right
            );

            if operator.is_none() && right.is_none() {
                return Ok(return_expr);
            }

            let error_fmt = errors
                .iter_mut()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            let operator = match operator {
                Some(value) => value,
                None => {
                    return Err(self.craft_error(format!("Expected operator: \n\t{}", error_fmt)));
                }
            };

            let right = match right {
                Some(value) => value,
                None => {
                    return Err(self.craft_error(format!("Expected right side: \n\t{}", error_fmt)));
                }
            };

            return_expr = Binary::create(return_expr, operator, right);
        }
    }
}
