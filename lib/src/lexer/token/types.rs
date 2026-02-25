use std::collections::HashMap;

use once_cell::sync::Lazy;

/// TokenTypes
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum TokenType {
    Literal(LiteralToken),
    Keyword(KeywordToken),
    Math(MathToken),
    Logic(LogicToken),

    Misc(MiscToken),
}

/// Tokens that are literals
#[derive(Debug, Clone, PartialOrd)]
pub enum LiteralToken {
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),
}

impl PartialEq for LiteralToken {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0.total_cmp(r0).is_eq(),
            _ => false,
        }
    }
}

impl Eq for LiteralToken {}

/// Tokens that don't fit in a Category
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum MiscToken {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
}

/// Tokens related to math
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum MathToken {
    Minus,
    Plus,
    Slash,
    Star,
}

/// Tokens that may be multiple Characters
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
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
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq)]
pub enum KeywordToken {
    And,
    Or,
    If,
    True,
    False,
    Let,
    In,
    With,
}

static ALL_KEYWORDS: Lazy<HashMap<&'static str, KeywordToken>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("and", KeywordToken::And);
    map.insert("or", KeywordToken::Or);
    map.insert("if", KeywordToken::If);
    map.insert("true", KeywordToken::True);
    map.insert("false", KeywordToken::False);
    map.insert("let", KeywordToken::Let);
    map.insert("in", KeywordToken::In);
    map.insert("with", KeywordToken::With);

    map
});
impl KeywordToken {
    pub fn is_keyword(input: &str) -> Option<KeywordToken> {
        ALL_KEYWORDS.get(input).copied()
    }
}
