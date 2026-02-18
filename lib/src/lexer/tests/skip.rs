#[cfg(test)]
mod test {
    use crate::lexer::{parse_text, tests::definition::LexerTest, token::Token};

    #[test]
    pub fn skip_single_space() {
        LexerTest::single_line_test(" ", vec![]);
    }

    #[test]
    pub fn skip_multi_space() {
        LexerTest::single_line_test("   ", vec![]);
    }

    #[test]
    pub fn skip_newline() {
        let input = "\n".to_string();

        let tokens = parse_text(input).unwrap();

        assert_eq!(tokens, vec![Token::get_eof_token(2)])
    }

    #[test]
    pub fn skip_comments() {
        let input = "#abdef".to_string();
        let tokens = parse_text(input).unwrap();

        assert_eq!(tokens, vec![Token::get_eof_token(1)])
    }
}
