use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Expr, Expression, Literal},
    },
    lexer::token::types::{LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_identifier(&mut self) -> Result<Expression, SyntaxError> {
        match self.next() {
            Some(token) => match token.r#type {
                TokenType::Literal(LiteralToken::Identifier(_)) => {
                    Ok(Literal::create(token.clone()))
                }

                _ => Err(SyntaxError::SyntaxError {
                    line: token.line,
                    column: token.column,
                    msg: "Expected Identifier".to_string(),
                }),
            },
            None => Err(SyntaxError::SyntaxError {
                line: 0,
                column: 0,
                msg: "Expected Identifier".to_string(),
            }),
        }
    }
}
