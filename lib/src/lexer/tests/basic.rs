#[cfg(test)]
mod test {
    use crate::lexer::{
        tests::definition::LexerTest,
        token::types::{MathToken, MiscToken, TokenType},
    };

    #[test]
    pub fn left_parent() {
        LexerTest::expect_single_token("(", TokenType::Misc(MiscToken::LeftParen));
    }

    #[test]
    pub fn right_parent() {
        LexerTest::expect_single_token(")", TokenType::Misc(MiscToken::RightParen));
    }

    #[test]
    pub fn left_brace() {
        LexerTest::expect_single_token("{", TokenType::Misc(MiscToken::LeftBrace));
    }

    #[test]
    pub fn right_brace() {
        LexerTest::expect_single_token("}", TokenType::Misc(MiscToken::RightBrace));
    }

    #[test]
    pub fn left_bracket() {
        LexerTest::expect_single_token("[", TokenType::Misc(MiscToken::LeftBracket));
    }

    #[test]
    pub fn right_bracket() {
        LexerTest::expect_single_token("]", TokenType::Misc(MiscToken::RightBracket));
    }

    #[test]
    pub fn comma() {
        LexerTest::expect_single_token(",", TokenType::Misc(MiscToken::Comma));
    }

    #[test]
    pub fn dot() {
        LexerTest::expect_single_token(".", TokenType::Misc(MiscToken::Dot));
    }

    #[test]
    pub fn semicolon() {
        LexerTest::expect_single_token(";", TokenType::Misc(MiscToken::Semicolon));
    }

    #[test]
    pub fn plus() {
        LexerTest::expect_single_token("+", TokenType::Math(MathToken::Plus));
    }

    #[test]
    pub fn minus() {
        LexerTest::expect_single_token("-", TokenType::Math(MathToken::Minus));
    }

    #[test]
    pub fn slash() {
        LexerTest::expect_single_token("/", TokenType::Math(MathToken::Slash));
    }

    #[test]
    pub fn star() {
        LexerTest::expect_single_token("*", TokenType::Math(MathToken::Star));
    }
}
