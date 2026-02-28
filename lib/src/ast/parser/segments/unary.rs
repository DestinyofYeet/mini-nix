use itertools::Itertools;
use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::SyntaxError},
        types::{Expression, Unary},
    },
    lexer::token::types::{LogicToken, MathToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_unary(&mut self) -> ParseResult {
        trace!("parse_unary");

        let mut errors: Vec<SyntaxError> = Vec::new();

        let mut final_expr: Option<Expression> = None;

        if let Some(token) = self.is_match(&[
            TokenType::Logic(LogicToken::Not),
            TokenType::Math(MathToken::Minus),
        ]) {
            match self.parse_unary() {
                Ok(value) => {
                    final_expr = Some(Unary::create(token, value));
                }
                Err(e) => {
                    errors.push(e);
                    self.revert();
                }
            }
        }

        if final_expr.is_none() {
            match self.parse_primary_or_identifier() {
                Ok(value) => {
                    final_expr = Some(value);
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        match final_expr {
            None => Err(self.craft_error(format!(
                "Expected '!', '-', Primary or Identifier:\n\t{}",
                errors.into_iter().map(|e| e.to_string()).join("\n")
            ))),
            Some(value) => {
                trace!("expr: {value:?}");
                Ok(value)
            }
        }
    }
}
