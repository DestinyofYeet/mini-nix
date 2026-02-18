use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Expression, Literal},
    },
    lexer::token::types::{LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_numbers(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        let mut errors = Vec::<SyntaxError>::new();

        let left = match self.next() {
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
                    msg: "Expected number".to_string(),
                });

                None
            }

            Some(token) => match &token.r#type {
                TokenType::Literal(literal) => match literal {
                    LiteralToken::Integer(_) | LiteralToken::Float(_) => {
                        Some(Literal::create(token.clone()))
                    }

                    _ => {
                        errors.push(SyntaxError::SyntaxError {
                            line: token.line,
                            column: token.column,
                            msg: "Expected number.".to_string(),
                        });
                        None
                    }
                },

                _ => {
                    errors.push(SyntaxError::SyntaxError {
                        line: token.line,
                        column: token.column,
                        msg: "Expected number".to_string(),
                    });
                    None
                }
            },
        };

        match left {
            None => Err(errors),
            Some(token) => Ok(token),
        }
    }
}
