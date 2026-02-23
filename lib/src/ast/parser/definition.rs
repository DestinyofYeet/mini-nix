use tracing::trace;

use crate::{
    ast::{
        parser::error::SyntaxError,
        types::{Expr, Expression},
    },
    lexer::token::{
        Token,
        types::{MiscToken, TokenType},
    },
};

/// This will take tokens and generate an AST
pub struct AstParser {
    tokens: Vec<Token>,
    index: usize,
}

pub type ParseResult = Result<Expression, Vec<SyntaxError>>;

#[allow(dead_code)]
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
        trace!("current. value: {:?}", self.tokens.get(self.index));
        self.tokens.get(self.index)
    }

    pub fn previous(&self) -> Option<&Token> {
        if self.index == 0 {
            return None;
        }

        self.tokens.get(self.index - 1)
    }

    pub fn advance(&mut self) {
        trace!("advance. old: {}, new: {}", self.index, self.index + 1);
        self.index += 1;
    }

    pub fn revert(&mut self) {
        self.index -= 1;
    }

    /// Returns the current value and advances
    pub fn next(&mut self) -> Option<&Token> {
        trace!(
            "next. current-value: {:?}, next-value: {:?}, next-index: {}",
            self.current(),
            self.tokens.get(self.index + 1),
            self.index + 1
        );
        let current = self.tokens.get(self.index);
        self.index += 1;

        current
    }

    pub fn next_cloned(&mut self) -> Option<Token> {
        self.next().cloned()
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

    pub fn craft_error(&self, message: impl ToString) -> SyntaxError {
        let mut line = 0;
        let mut column = 0;

        if let Some(current) = self.current() {
            line = current.line;
            column = current.column;
        }

        SyntaxError::SyntaxError {
            line,
            column,
            msg: message.to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<Expression, Vec<SyntaxError>> {
        let mut errors = Vec::<SyntaxError>::new();

        let expr = match self.parse_expression() {
            Ok(expr) => Some(expr),
            Err(mut e) => {
                errors.append(&mut e);
                None
            }
        };

        trace!("expr: {:?}", expr);
        match expr {
            None => Err(errors),
            Some(value) => Ok(value),
        }
    }
}
