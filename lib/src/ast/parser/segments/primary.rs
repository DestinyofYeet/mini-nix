use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::SyntaxError},
        types::{Expression, Grouping, Literal},
    },
    lexer::token::types::{KeywordToken, LiteralToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_primary(&mut self) -> ParseResult {
        trace!("parse_primary");

        let result = match self.next_cloned() {
            None => {
                self.revert();
                return Err(SyntaxError::OutOfTokens);
            }

            Some(token) => match &token.r#type {
                TokenType::Keyword(KeywordToken::True)
                | TokenType::Keyword(KeywordToken::False)
                | TokenType::Literal(LiteralToken::Float(_))
                | TokenType::Literal(LiteralToken::String(_))
                | TokenType::Literal(LiteralToken::Integer(_)) => Literal::create(token.clone()),

                TokenType::Misc(MiscToken::LeftParen) => {
                    let expression = Grouping::create(self.parse_expression_no_assignment()?);

                    if self
                        .is_match(&[TokenType::Misc(MiscToken::RightParen)])
                        .is_none()
                    {
                        self.revert();
                        return Err(self.craft_error("Expected ')'"));
                    }

                    expression
                }

                _ => {
                    self.revert();
                    return Err(self.craft_error("Expected string, float or integer"));
                }
            },
        };

        trace!("expr: {result:?}");
        Ok(result)
    }
}
