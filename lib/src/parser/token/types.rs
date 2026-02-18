use std::collections::HashMap;

use once_cell::sync::Lazy;

/// TokenTypes
#[derive(Debug)]
pub enum TokenType {
    Literal(LiteralToken),
    Keyword(KeywordToken),
    Math(MathToken),
    Logic(LogicToken),

    Misc(MiscToken),
}

/// Tokens that are literals
#[derive(Debug)]
pub enum LiteralToken {
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),
}

/// Tokens that don't fit in a Category
#[derive(Debug)]
pub enum MiscToken {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Comment,

    Eof,
}

/// Tokens related to math
#[derive(Debug)]
pub enum MathToken {
    Minus,
    Plus,
    Slash,
    Star,
}

/// Tokens that may be multiple Characters
#[derive(Debug)]
pub enum LogicToken {
    Not,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

/// Tokens that are keywords
#[derive(Debug, Copy, Clone)]
pub enum KeywordToken {
    And,
    Or,
    If,
    True,
    False,
}

static ALL_KEYWORDS: Lazy<HashMap<&'static str, KeywordToken>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("and", KeywordToken::And);
    map.insert("or", KeywordToken::Or);
    map.insert("if", KeywordToken::If);
    map.insert("true", KeywordToken::True);
    map.insert("false", KeywordToken::False);

    map
});
impl KeywordToken {
    pub fn is_keyword(input: &str) -> Option<KeywordToken> {
        ALL_KEYWORDS.get(input).copied()
    }
}
