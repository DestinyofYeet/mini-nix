#[cfg(test)]
mod test {
    use crate::parser::{
        tests::definition::ParserTest,
        token::types::{LogicToken, TokenType},
    };

    #[test]
    pub fn not() {
        ParserTest::expect_single_token("!", TokenType::Logic(LogicToken::Not));
    }

    #[test]
    pub fn not_eq() {
        ParserTest::expect_single_token("!=", TokenType::Logic(LogicToken::NotEqual));
    }

    #[test]
    pub fn eq() {
        ParserTest::expect_single_token("=", TokenType::Logic(LogicToken::Equal));
    }

    #[test]
    pub fn less() {
        ParserTest::expect_single_token("<", TokenType::Logic(LogicToken::Less));
    }

    #[test]
    pub fn less_equal() {
        ParserTest::expect_single_token("<=", TokenType::Logic(LogicToken::LessEqual));
    }

    #[test]
    pub fn greater() {
        ParserTest::expect_single_token(">", TokenType::Logic(LogicToken::Greater));
    }

    #[test]
    pub fn greater_equal() {
        ParserTest::expect_single_token(">=", TokenType::Logic(LogicToken::GreaterEqual));
    }
}
