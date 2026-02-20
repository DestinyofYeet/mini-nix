use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Expr, Expression, Literal},
    },
    lexer::token::types::{LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_identifier(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        trace!("parse_identifier");
        let mut errors = Vec::<SyntaxError>::new();

        let result = match self.next() {
            Some(token) => match token.r#type {
                TokenType::Literal(LiteralToken::Identifier(_)) => {
                    Some(Literal::create(token.clone()))
                }

                _ => {
                    errors.push(SyntaxError::SyntaxError {
                        line: token.line,
                        column: token.column,
                        msg: "Expected Identifier".to_string(),
                    });
                    None
                }
            },
            None => {
                errors.push(SyntaxError::SyntaxError {
                    line: 0,
                    column: 0,
                    msg: "Expected Identifier".to_string(),
                });
                self.revert();
                None
            }
        };

        if result.is_none() {
            return Err(errors);
        }

        Ok(result.unwrap())
    }
}
