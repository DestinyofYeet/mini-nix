use tracing::trace;

use crate::{
    ast::{
        parser::{AstParser, ParseResult, error::AstError},
        types::{Expression, Grouping, Literal},
    },
    lexer::token::{
        Token,
        types::{KeywordToken, LiteralToken, MiscToken, TokenType},
    },
};

impl AstParser {
    pub fn parse_primary(&mut self) -> ParseResult {
        trace!("parse_primary");
        let mut revert_by: u64 = 0;

        let result: Result<Expression, AstError> = match self.next().cloned() {
            None => return Err(AstError::OutOfTokens.into()),

            Some(token) => match &token.r#type {
                TokenType::Literal(LiteralToken::String(_))
                | TokenType::Literal(LiteralToken::Integer(_))
                | TokenType::Literal(LiteralToken::Float(_))
                | TokenType::Keyword(KeywordToken::True)
                | TokenType::Keyword(KeywordToken::False) => Ok(Literal::new(token)),

                TokenType::Misc(MiscToken::LeftParen) => {
                    revert_by += 1;

                    match self.parse_expression_no_assignemnt() {
                        Err(errors) => {
                            Err(self.craft_error("Failed to parse grouping because:", errors))
                        }
                        Ok(expression) => {
                            match self.is_match(&[TokenType::Misc(MiscToken::RightParen)]) {
                                None => Err(self.craft_error("Expected ')'", None)),
                                Some(_) => Ok(Grouping::new(expression)),
                            }
                        }
                    }
                }

                value => {
                    revert_by += 1;

                    Err(self.craft_error(format!("Expected primary, found {value}"), None))
                }
            },
        };

        trace!("result: {result:?}");
        let primary_error = match result {
            Ok(value) => return Ok(value),
            Err(e) => e,
        };

        self.revert_by(revert_by);

        // todo!("parse list");

        Err(vec![primary_error])
    }
}
