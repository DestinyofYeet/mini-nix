use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::AstError},
        types::{Expression, Literal},
    },
    lexer::token::types::{LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_primary_or_identifier(&mut self) -> ParseResult {
        trace!("parse_primary_or_identifier");

        let identifier: Result<Expression, Vec<AstError>> = match self.next().cloned() {
            None => {
                self.revert();
                return Err(AstError::OutOfTokens.into());
            }

            Some(token) => match token.r#type {
                TokenType::Literal(LiteralToken::Identifier(_)) => Ok(Literal::new(token.clone())),

                value => {
                    self.revert();
                    Err(self
                        .craft_error(format!("Expected Identifier, found {value}"), None)
                        .into())
                }
            },
        };

        let identifier_err: AstError = match identifier {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse identifier because:", errors),
        };

        let primary_err: AstError = match self.parse_primary() {
            Ok(value) => return Ok(value),
            Err(errors) => self.craft_error("Failed to parse primary because:", errors),
        };

        Err(vec![identifier_err, primary_err])
    }
}
