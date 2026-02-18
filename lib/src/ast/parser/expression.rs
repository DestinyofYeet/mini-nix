use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Expr, Expression},
    },
    lexer::token::types::{LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_expression(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        let mut errors: Vec<SyntaxError> = Vec::new();

        if let Some(token) = self.current() {
            if let TokenType::Literal(LiteralToken::Identifier(_)) = &token.r#type {
                match self.parse_assignment() {
                    Ok(expr) => return Ok(expr),
                    Err(e) => {
                        errors.push(e);
                        return Err(errors);
                    }
                }
            }

            if let TokenType::Literal(value) = &token.r#type {
                match value {
                    LiteralToken::Integer(_) | LiteralToken::Float(_) => {
                        match self.parse_arithmetic() {
                            Ok(expr) => return Ok(expr),
                            Err(mut e) => {
                                errors.append(&mut e);
                                return Err(errors);
                            }
                        }
                    }

                    _ => unreachable!(),
                }
            }
        }

        errors.push({
            SyntaxError::SyntaxError {
                line: 0,
                column: 0,
                msg: "Expected identifier or value".to_string(),
            }
        });

        Err(errors)
    }
}
