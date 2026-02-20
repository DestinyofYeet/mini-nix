use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::{Expression, Grouping, Literal},
    },
    lexer::token::types::{KeywordToken, LiteralToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_primary(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        trace!("parse_primary");
        let mut errors = Vec::<SyntaxError>::new();

        let result = match self.next_cloned() {
            None => {
                errors.push(SyntaxError::SyntaxError {
                    line: 0,
                    column: 0,
                    msg: "Expected token".to_string(),
                });

                None
            }

            Some(token) => match &token.r#type {
                TokenType::Keyword(KeywordToken::True)
                | TokenType::Keyword(KeywordToken::False)
                | TokenType::Literal(LiteralToken::Float(_))
                | TokenType::Literal(LiteralToken::String(_))
                | TokenType::Literal(LiteralToken::Integer(_)) => {
                    Some(Literal::create(token.clone()))
                }

                TokenType::Misc(MiscToken::LeftParen) => {
                    let expression = match self.parse_expression() {
                        Ok(value) => Some(value),
                        Err(mut e) => {
                            errors.append(&mut e);
                            None
                        }
                    };

                    let right_match = self.is_match(&[TokenType::Misc(MiscToken::RightParen)]);

                    if right_match.is_none() {
                        errors.push(SyntaxError::SyntaxError {
                            line: token.line,
                            column: token.column,
                            msg: "Expected ')'".to_string(),
                        });
                        return Err(errors);
                    }

                    expression.map(Grouping::create)
                }

                _ => {
                    errors.push(SyntaxError::SyntaxError {
                        line: token.line,
                        column: token.column,
                        msg: "Expected string, float or integer".to_string(),
                    });
                    None
                }
            },
        };

        match result {
            None => {
                self.revert();
                Err(errors)
            }
            Some(value) => {
                trace!("expr: {:?}", value);
                Ok(value)
            }
        }
    }
}
