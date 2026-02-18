use crate::{
    ast::{
        parser::error::SyntaxError,
        types::{Expr, Literal},
    },
    lexer::token::{
        Token,
        types::{LiteralToken, LogicToken, TokenType},
    },
};

/// This will take tokens and generate an AST
/// Grammar (non-exhaustive):
/// expression -> (assignment | primary) ';'
/// assignment -> identifier '=' primary
/// primary    -> number | string | bool
pub struct AstParser {
    tokens: Vec<Token>,
    index: usize,
}
impl AstParser {
    pub fn new(tokenstream: Vec<Token>) -> Self {
        Self {
            tokens: tokenstream,
            index: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn previous(&self) -> Option<&Token> {
        if self.index == 0 {
            return None;
        }

        self.tokens.get(self.index - 1)
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    /// Returns the current value and advances
    pub fn next(&mut self) -> Option<&Token> {
        let current = self.tokens.get(self.index);
        self.index += 1;

        current
    }

    pub fn is_match(&mut self, tokens: &[TokenType]) -> Option<Token> {
        let mut returned = None;

        if let Some(current) = self.current() {
            for token in tokens {
                if *token == current.r#type {
                    returned = Some(current.clone());
                    break;
                }
            }
        }

        if returned.is_some() {
            self.advance();
        }

        returned
    }

    pub fn parse(&mut self) -> Result<impl Expr, Vec<SyntaxError>> {
        let mut errors: Vec<SyntaxError> = Vec::new();

        let res = match self.parse_assignment() {
            Ok(expr) => Some(expr),
            Err(e) => {
                errors.push(e);
                None
            }
        };

        if res.is_none() {
            return Err(errors);
        }

        return Ok(res.unwrap());
    }
}
