use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Expr, Expression},
    },
    lexer::token::types::{LiteralToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_expression(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        trace!("parse_expression");
        let mut errors: Vec<SyntaxError> = Vec::new();

        let expr = match self.current() {
            None => {
                errors.push(SyntaxError::OutOfTokens);
                None
            }

            Some(token) => match token.r#type {
                TokenType::Literal(LiteralToken::Identifier(_)) => match self.parse_assignment() {
                    Ok(expr) => Some(expr),
                    Err(mut e) => {
                        errors.append(&mut e);
                        None
                    }
                },

                TokenType::Literal(LiteralToken::Integer(_))
                | TokenType::Literal(LiteralToken::Float(_))
                | TokenType::Misc(MiscToken::LeftParen) => match self.parse_arithmetic() {
                    Ok(expr) => Some(expr),
                    Err(mut e) => {
                        errors.append(&mut e);
                        None
                    }
                },

                _ => todo!("fuck"),
            },
        };

        trace!("expr: {:?}", expr);
        match expr {
            None => Err(errors),
            Some(expr) => Ok(expr),
        }
    }
}
