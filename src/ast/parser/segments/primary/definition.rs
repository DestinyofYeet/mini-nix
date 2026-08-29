use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::AstError},
        types::{Expression, Grouping, Literal},
    },
    lexer::token::types::{KeywordToken, LiteralToken, MiscToken, TokenType},
};

impl AstParser {
    pub fn parse_primary(&mut self) -> ParseResult {
        trace!("parse_primary");

        let pre_index = self.index;

        let result: Result<Expression, AstError> = match self.next().cloned() {
            None => {
                self.set_index(pre_index);
                return Err(AstError::OutOfTokens.into());
            }

            Some(token) => match &token.r#type {
                TokenType::Literal(LiteralToken::String(_))
                | TokenType::Literal(LiteralToken::Integer(_))
                | TokenType::Literal(LiteralToken::Float(_))
                | TokenType::Keyword(KeywordToken::True)
                | TokenType::Keyword(KeywordToken::False) => Ok(Literal::new(token)),

                TokenType::Misc(MiscToken::LeftParen) => {
                    match self.parse_expression_no_assignemnt() {
                        Err(errors) => {
                            self.set_index(pre_index);
                            Err(self.craft_error("Failed to parse grouping because:", errors))
                        }
                        Ok(expression) => {
                            match self.is_match(&[TokenType::Misc(MiscToken::RightParen)]) {
                                None => {
                                    self.set_index(pre_index);
                                    Err(self.craft_error("Expected ')'", None))
                                }
                                Some(_) => Ok(Grouping::new(expression)),
                            }
                        }
                    }
                }

                value => {
                    self.set_index(pre_index);

                    Err(self.craft_error(format!("Expected primary, found {value}"), None))
                }
            },
        };

        trace!("result: {result:?}");
        let primary_error = match result {
            Ok(value) => return Ok(value),
            Err(e) => e,
        };

        // todo!("parse list");

        Err(vec![primary_error])
    }
}
