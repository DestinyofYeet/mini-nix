use crate::{
    ast::{
        parser::{AstParser, error::SyntaxError},
        types::Literal,
    },
    lexer::token::types::{KeywordToken, LiteralToken, TokenType},
};

impl AstParser {
    pub fn parse_primary(&mut self) -> Result<Literal, SyntaxError> {
        if let Some(value) = self.next() {
            let result = match value.r#type {
                TokenType::Keyword(KeywordToken::True)
                | TokenType::Keyword(KeywordToken::False)
                | TokenType::Literal(LiteralToken::String(_))
                | TokenType::Literal(LiteralToken::Integer(_))
                | TokenType::Literal(LiteralToken::Float(_)) => Literal {
                    literal: value.clone(),
                },

                _ => {
                    return Err(SyntaxError::SyntaxError {
                        line: value.line,
                        column: value.column,
                        msg: "Expected value".to_string(),
                    });
                }
            };

            Ok(result)
        } else {
            Err(SyntaxError::SyntaxError {
                line: 0,
                column: 0,
                msg: "Expected value".to_string(),
            })
        }
    }
}
