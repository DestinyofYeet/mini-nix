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
    pub fn assignment() {
        let tokens = parse_text("a = 2;".to_string()).unwrap();

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
    pub fn assignment_and_arithmetic() {
        let tokens = parse_text("a = 1+2+3;".to_string()).unwrap();

        let mut parser = AstParser::new(tokens.clone());

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Binary::create(
                Literal::create(tokens[0].clone()),
                tokens[1].clone(),
                Binary::create(
                    Binary::create(
                        Literal::create(tokens[2].clone()),
                        tokens[3].clone(),
                        Literal::create(tokens[4].clone())
                    ),
                    tokens[5].clone(),
                    Literal::create(tokens[6].clone())
                )
            )
        )
    }
}
