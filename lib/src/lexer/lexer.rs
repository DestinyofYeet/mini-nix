use tracing::trace;

use crate::lexer::{
    error::ParserError,
    token::{
        Token,
        types::{KeywordToken, LiteralToken, LogicToken, MathToken, MiscToken, TokenType},
    },
};

pub fn parse_text(source: String) -> Result<Vec<Token>, Vec<ParserError>> {
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

    let index_source = |start: usize, end: usize| -> Result<&str, ParserError> {
        match source.get(start..end) {
            Some(res) => Ok(res),
            None => Err(ParserError::WrongIndex { start, end }),
        }
    };

    while let Some((char_index, char)) = iterator.next() {
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
                trace!("Skipped space");
                continue;
            }

            '(' => TokenType::Misc(MiscToken::LeftParen),
            ')' => TokenType::Misc(MiscToken::RightParen),
            '{' => TokenType::Misc(MiscToken::LeftBrace),
            '}' => TokenType::Misc(MiscToken::RightBrace),
            '[' => TokenType::Misc(MiscToken::LeftBracket),
            ']' => TokenType::Misc(MiscToken::RightBracket),
            '#' => {
                loop {
                    let (_, next) = match iterator.next() {
                        None => break,
                        Some(value) => value,
                    };

                    if next == '\n' {
                        advance_line();
                        break;
                    } else {
                        continue;
                    }
                }
                continue;
            }
            ',' => TokenType::Misc(MiscToken::Comma),
            '.' => TokenType::Misc(MiscToken::Dot),
            ';' => TokenType::Misc(MiscToken::Semicolon),

            '+' => TokenType::Math(MathToken::Plus),
            '-' => TokenType::Math(MathToken::Minus),
            '/' => TokenType::Math(MathToken::Slash),
            '*' => TokenType::Math(MathToken::Star),

            '!' => {
                if match_next("=", char_index) {
                    token_width = 2;
                    iterator.next();

                    TokenType::Logic(LogicToken::NotEqual)
                } else {
                    TokenType::Logic(LogicToken::Not)
                }
            }

            '=' => TokenType::Logic(LogicToken::Equal),
            '<' => {
                if match_next("=", char_index) {
                    token_width = 2;
                    iterator.next();
                    TokenType::Logic(LogicToken::LessEqual)
                } else {
                    TokenType::Logic(LogicToken::Less)
                }
            }

            '>' => {
                if match_next("=", char_index) {
                    token_width = 2;
                    iterator.next();
                    TokenType::Logic(LogicToken::GreaterEqual)
                } else {
                    TokenType::Logic(LogicToken::Greater)
                }
            }

            '0'..='9' => {
                let mut is_float = false;
                let mut index = char_index;
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
                    is_float = true;
                    iterator.next();
                    index += 1;
                }

                let index_copy = index;

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

                let mut malformed_float = false;

                // we have a malformed float (1.)
                if index == index_copy && is_float {
                    malformed_float = true;
                }

                let value = {
                    let index = if malformed_float { index - 1 } else { index };
                    match index_source(char_index, index + 1) {
                        Ok(value) => value.to_string(),
                        Err(e) => {
                            errors.push(e);
                            continue;
                        }
                    }
                };

                token_width = index - char_index + 1;

                if is_float && !malformed_float {
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

            'a'..='z' | 'A'..='Z' => {
                let mut index = char_index;
                loop {
                    let (i, next) = match iterator.peek() {
                        None => break,
                        Some(value) => value,
                    };

                    if next.is_ascii_alphanumeric() {
                        index = *i;
                        iterator.next();
                    } else {
                        break;
                    }
                }

                let value = match index_source(char_index, index + 1) {
                    Ok(val) => val.to_string(),
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                // this is an index, but we need to know how many chars are in it (e.g. index 2 => 3 elements)
                token_width = index - char_index + 1;

                match KeywordToken::is_keyword(&value) {
                    Some(keyword) => TokenType::Keyword(keyword),
                    None => TokenType::Literal(LiteralToken::Identifier(value)),
                }
            }

            '"' => {
                let mut index = char_index;
                while let Some((i, next)) = iterator.next()
                    && next != '"'
                {
                    if next == '\n' {
                        advance_line()
                    }

                    index = i;
                }

                // + 1 checks for a closing quote
                if is_at_end(index) || is_at_end(index + 1) {
                    errors.push(ParserError::UnfinishedString { line, column });

                    continue;
                }

                let value = match index_source(char_index + 1, index + 1) {
                    Ok(val) => val.to_string(),
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                // we want to capture the "raw" string. +2 for the quotes
                token_width = index - char_index + 2;

                TokenType::Literal(LiteralToken::String(value))
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

        let source_value = match index_source(char_index, char_index + token_width) {
            Ok(e) => e.to_string(),
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        tokens.push(Token::new(token_type, source_value, line, column));
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(tokens)
}
