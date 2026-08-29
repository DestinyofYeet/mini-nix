#[cfg(test)]
mod test {
    use crate::lexer::{
        tests::definition::LexerTest,
        token::types::{LogicToken, TokenType},
    };

    #[test]
    pub fn not() {
        LexerTest::expect_single_token("!", TokenType::Logic(LogicToken::Not));
    }

    #[test]
    pub fn not_eq() {
        LexerTest::expect_single_token("!=", TokenType::Logic(LogicToken::NotEqual));
    }

    #[test]
    pub fn eq() {
        LexerTest::expect_single_token("=", TokenType::Logic(LogicToken::Equal));
    }

    #[test]
    pub fn less() {
        LexerTest::expect_single_token("<", TokenType::Logic(LogicToken::Less));
    }

    #[test]
    pub fn less_equal() {
        LexerTest::expect_single_token("<=", TokenType::Logic(LogicToken::LessEqual));
    }

    #[test]
    pub fn greater() {
        LexerTest::expect_single_token(">", TokenType::Logic(LogicToken::Greater));
    }

    #[test]
    pub fn greater_equal() {
        LexerTest::expect_single_token(">=", TokenType::Logic(LogicToken::GreaterEqual));
    }
}
