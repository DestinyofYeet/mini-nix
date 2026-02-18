use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Binary, Expr, Literal},
    },
    lexer::token::{
        Token,
        types::{LiteralToken, LogicToken, TokenType},
    },
};

impl AstParser {
    pub fn parse_assignment(&mut self) -> Result<impl Expr, SyntaxError> {
        let expr = match self.next() {
            Some(token) => match token.r#type {
                TokenType::Literal(LiteralToken::Identifier(_)) => Literal {
                    literal: token.clone(),
                },

                _ => {
                    return Err(SyntaxError::SyntaxError {
                        line: token.line,
                        column: token.column,
                        msg: "Expected Identifier".to_string(),
                    });
                }
            },
            None => {
                return Err(SyntaxError::SyntaxError {
                    line: 0,
                    column: 0,
                    msg: "Expected Identifier".to_string(),
                });
            }
        };

        let operator = match self.is_match(&[TokenType::Logic(LogicToken::Equal)]) {
            Some(token) => token,

            None => {
                return Err(SyntaxError::SyntaxError {
                    line: expr.literal.line,
                    column: expr.literal.column,
                    msg: "= expected".to_string(),
                });
            }
        };

        let primary = self.parse_primary()?;

        Ok(Binary {
            left: expr,
            operator,
            right: primary,
        })
    }
}
