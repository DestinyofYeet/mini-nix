use crate::{
    ast::{parser::error::SyntaxError, types::Expr},
    lexer::token::{
        Token,
        types::{MiscToken, TokenType},
    },
};

/// This will take tokens and generate an AST
/// Grammar (non-exhaustive):
/// start         -> expression ';'
/// expression    -> (assignment | primary | arithmetic)
/// assignment    -> identifier '=' primary
/// primary       -> numbers | string | bool | '(' expression ')'
/// arithmetic    -> arithmeticMul ( ( "-" | "+" ) arithmetic )*
/// arithmeticMul -> numbers ( ( "/" | "*" ) arithmeticMul )*
/// numbers       -> float | int
pub struct AstParser {
    tokens: Vec<Token>,
    index: usize,
}

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
        let mut errors = Vec::<SyntaxError>::new();

        match self.parse_expression() {
            Err(mut e) => {
                errors.append(&mut e);

                Err(errors)
            }
            Ok(value) => {
                if let Some(token) = self.next()
                    && token.r#type != TokenType::Misc(MiscToken::Semicolon)
                {
                    errors.push(SyntaxError::SyntaxError {
                        line: token.line,
                        column: token.column,
                        msg: "Expected ';'".to_string(),
                    });

                    return Err(errors);
                }

                Ok(value)
            }
        }
    }
}
