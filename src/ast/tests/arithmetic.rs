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
        let tokens = parse_text("1 + 1").unwrap();

        let mut parser = AstParser::new(tokens.clone());

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Binary::new(
                Literal::new(tokens[0].clone()),
                tokens[1].clone(),
                Literal::new(tokens[2].clone())
            )
        )
    }

    #[test]
    pub fn one_plue_one_plus_one() {
        let tokens = parse_text("1 - 1 + 1").unwrap();

        let mut parser = AstParser::new(tokens.clone());

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Binary::new(
                Binary::new(
                    Literal::new(tokens[0].clone()),
                    tokens[1].clone(),
                    Literal::new(tokens[2].clone())
                ),
                tokens[3].clone(),
                Literal::new(tokens[4].clone())
            )
        )
    }
}
