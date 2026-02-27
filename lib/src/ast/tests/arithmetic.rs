#[cfg(test)]
mod test {
    use crate::{
        ast::{
            parser::AstParser,
            types::{Binary, Literal},
        },
        lexer::parse_text,
    };

    #[test]
    pub fn one_plus_one() {
        let tokens = parse_text("1 + 1".to_string()).unwrap();

        let mut parser = AstParser::new(tokens.clone());

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Binary::create(
                Literal::create(tokens[0].clone()),
                tokens[1].clone(),
                Literal::create(tokens[2].clone())
            )
        )
    }

    #[test]
    pub fn one_plue_one_plus_one() {
        let tokens = parse_text("1 - 1 + 1".to_string()).unwrap();

        let mut parser = AstParser::new(tokens.clone());

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Binary::create(
                Binary::create(
                    Literal::create(tokens[0].clone()),
                    tokens[1].clone(),
                    Literal::create(tokens[2].clone())
                ),
                tokens[3].clone(),
                Literal::create(tokens[4].clone())
            )
        )
    }
}
