use tracing::debug;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Binary, Expr, Expression, Literal},
    },
    lexer::token::{
        Token,
        types::{MathToken, TokenType},
    },
};

impl AstParser {
    pub fn parse_arithmetic(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        let mut errors = Vec::<SyntaxError>::new();

        let left = match self.parse_numbers() {
            Ok(expr) => Some(expr),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        let operator = match self.is_match(&[
            TokenType::Math(MathToken::Minus),
            TokenType::Math(MathToken::Plus),
            TokenType::Math(MathToken::Slash),
            TokenType::Math(MathToken::Star),
        ]) {
            Some(token) => Some(token),
            None => {
                let mut line = 0;
                let mut column = 0;

                if let Some(current) = self.current() {
                    line = current.line;
                    column = current.column;
                }
                errors.push(SyntaxError::SyntaxError {
                    line,
                    column,
                    msg: "Expected operator".to_string(),
                });

                None
            }
        };

        let right = match self.parse_numbers() {
            Ok(expr) => Some(expr),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        if left.is_none() | operator.is_none() | right.is_none() {
            return Err(errors);
        }

        Ok(Binary::create(
            left.unwrap(),
            operator.unwrap(),
            right.unwrap(),
        ))
    }
}
