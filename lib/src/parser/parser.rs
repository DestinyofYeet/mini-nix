use tracing::trace;

use crate::parser::{
    error::ParserError,
    token::{
        Token,
        types::{LiteralToken, LogicToken, MathToken, MiscToken, TokenType},
    },
};

pub fn parse_text(source: String) -> Result<(), Vec<ParserError>> {
    let mut line: usize = 1;
    let mut column: usize = 0;

    let mut tokens: Vec<Token> = Vec::new();

    let mut errors = Vec::new();

    let mut iterator = source.char_indices().peekable();
    let iterator_max = iterator.clone().count();

    let match_next = |expected: &'static str, index: usize| {
        if let Some(next) = source.get(index + 1..index + expected.len() + 1)
            && next == expected
        {
            true
        } else {
            false
        }
    };

    let is_at_end = |index: usize| index >= iterator_max;

    while let Some((char_location, char)) = iterator.next() {
        let mut token_width = 1;

        column += 1;
        let mut advance_line = || {
            line += 1;
            column = 0;
        };

        let token_type = match char {
            '\n' => {
                advance_line();
                continue;
            }

            ' ' => {
                continue;
            }

            '(' => TokenType::Misc(MiscToken::LeftParen),
            ')' => TokenType::Misc(MiscToken::RightParen),
            '{' => TokenType::Misc(MiscToken::LeftBrace),
            '}' => TokenType::Misc(MiscToken::RightBrace),
            '#' => TokenType::Misc(MiscToken::Comment),
            ',' => TokenType::Misc(MiscToken::Comma),
            '.' => TokenType::Misc(MiscToken::Dot),
            ';' => TokenType::Misc(MiscToken::Semicolon),

            '+' => TokenType::Math(MathToken::Plus),
            '-' => TokenType::Math(MathToken::Minus),
            '/' => TokenType::Math(MathToken::Slash),
            '*' => TokenType::Math(MathToken::Star),

            '!' => {
                if match_next("=", char_location) {
                    token_width = 2;
                    iterator.next();

                    TokenType::Logic(LogicToken::NotEqual)
                } else {
                    TokenType::Logic(LogicToken::Not)
                }
            }

            '=' => TokenType::Logic(LogicToken::Equal),
            '<' => {
                if match_next("=", char_location) {
                    token_width = 2;
                    iterator.next();
                    TokenType::Logic(LogicToken::LessEqual)
                } else {
                    TokenType::Logic(LogicToken::Less)
                }
            }

            '>' => {
                if match_next("=", char_location) {
                    token_width = 2;
                    iterator.next();
                    TokenType::Logic(LogicToken::GreaterEqual)
                } else {
                    TokenType::Logic(LogicToken::Greater)
                }
            }

            '0'..='9' => {
                let mut is_float = false;
                let mut index = 0;
                loop {
                    let (i, next) = match iterator.peek() {
                        None => break,
                        Some(value) => value,
                    };

                    if next.is_ascii_digit() {
                        index = *i;
                        iterator.next();
                    } else {
                        break;
                    }
                }

                if match_next(".", index) {
                    iterator.next();
                    index += 1;
                }

                loop {
                    let (i, next) = match iterator.peek() {
                        None => break,
                        Some(value) => value,
                    };

                    if next.is_ascii_digit() {
                        is_float = true;
                        index = *i;
                        iterator.next();
                    } else {
                        break;
                    }
                }

                let value = source.get(char_location..index + 1).unwrap().to_string();

                token_width = index - char_location;

                if is_float {
                    match value.parse::<f64>() {
                        Ok(float) => TokenType::Literal(LiteralToken::Float(float)),
                        Err(e) => {
                            errors.push(ParserError::Conversion {
                                value,
                                to_type: "f64".to_string(),
                                err: e.to_string(),
                            });

                            continue;
                        }
                    }
                } else {
                    match value.parse::<i64>() {
                        Ok(integer) => TokenType::Literal(LiteralToken::Integer(integer)),
                        Err(e) => {
                            errors.push(ParserError::Conversion {
                                value,
                                to_type: "i64".to_string(),
                                err: e.to_string(),
                            });
                            continue;
                        }
                    }
                }
            }

            '"' => {
                let mut index = 0;
                while let Some((i, next)) = iterator.next()
                    && next != '"'
                {
                    if next == '\n' {
                        advance_line()
                    }

                    index = i;
                }

                if is_at_end(index) {
                    errors.push(ParserError::UnfinishedString { line });

                    continue;
                }

                let value = source
                    .get(char_location + 1..index + 1)
                    .unwrap()
                    .to_string();

                // we want to capture the "raw" string. +2 for the quotes
                token_width = index - char_location + 2;

                TokenType::Literal(LiteralToken::Str(value))
            }

            _ => {
                errors.push(ParserError::UnrecognizedToken {
                    token: char.into(),
                    line,
                    column,
                });
                continue;
            }
        };

        tokens.push(Token::new(
            token_type,
            source
                .get(char_location..char_location + token_width)
                .unwrap()
                .to_string(),
            line,
        ));
    }

    tokens.push(Token::new(
        TokenType::Misc(MiscToken::Eof),
        String::new(),
        line,
    ));

    trace!("{tokens:?}");

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(())
}
