#[cfg(test)]
mod test {
    use crate::parser::{
        tests::definition::ParserTest,
        token::types::{MathToken, MiscToken, TokenType},
    };

    #[test]
    pub fn left_parent() {
        ParserTest::expect_single_token("(", TokenType::Misc(MiscToken::LeftParen));
    }

    #[test]
    pub fn right_parent() {
        ParserTest::expect_single_token(")", TokenType::Misc(MiscToken::RightParen));
    }

    #[test]
    pub fn left_brace() {
        ParserTest::expect_single_token("{", TokenType::Misc(MiscToken::LeftBrace));
    }

    #[test]
    pub fn right_brace() {
        ParserTest::expect_single_token("}", TokenType::Misc(MiscToken::RightBrace));
    }

    #[test]
    pub fn comma() {
        ParserTest::expect_single_token(",", TokenType::Misc(MiscToken::Comma));
    }

    #[test]
    pub fn dot() {
        ParserTest::expect_single_token(".", TokenType::Misc(MiscToken::Dot));
    }

    #[test]
    pub fn semicolon() {
        ParserTest::expect_single_token(";", TokenType::Misc(MiscToken::Semicolon));
    }

    #[test]
    pub fn plus() {
        ParserTest::expect_single_token("+", TokenType::Math(MathToken::Plus));
    }

    #[test]
    pub fn minus() {
        ParserTest::expect_single_token("-", TokenType::Math(MathToken::Minus));
    }

    #[test]
    pub fn slash() {
        ParserTest::expect_single_token("/", TokenType::Math(MathToken::Slash));
    }

    #[test]
    pub fn star() {
        ParserTest::expect_single_token("*", TokenType::Math(MathToken::Star));
    }
}
