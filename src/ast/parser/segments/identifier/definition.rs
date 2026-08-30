use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::AstError},
        types::{Expression, Literal},
    },
    lexer::token::types::{LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_identifier(&mut self) -> ParseResult {
        trace!("parse_identifier");

        let pre_idx = self.index;

        match self.next().cloned() {
            None => {
                self.set_index(pre_idx);
                Err(AstError::OutOfTokens.into())
            }

            Some(token) => match token.r#type {
                TokenType::Literal(LiteralToken::Identifier(_)) => {
                    Ok(Literal::new_expr(token.clone()))
                }

                value => {
                    self.set_index(pre_idx);
                    Err(self
                        .craft_error(format!("Expected Identifier, found {value}"), None)
                        .into())
                }
            },
        }
    }
}
