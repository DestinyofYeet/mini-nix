#[cfg(test)]
mod test {
    use crate::{
        ast::{
            parser::AstParser,
            types::{
                Binary, Expression, Literal, Preamble, PreambleLetIn, PreambleType, PreambleWith,
            },
        },
        lexer::parse_text,
    };

    #[test]
    pub fn let_in_primary() {
        let tokens = parse_text("let a = 2; in 1".to_string()).unwrap();

        let mut parser = AstParser::new(tokens.clone());

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Preamble::create(
                PreambleType::LetIn(PreambleLetIn {
                    expr: vec![Binary::create(
                        Literal::create(tokens[1].clone()),
                        tokens[2].clone(),
                        Literal::create(tokens[3].clone())
                    )]
                }),
                Literal::create(tokens[6].clone())
            )
        )
    }

    #[test]
    pub fn let_in_assignment() {
        let tokens = parse_text("let a = 2; in b = a;".to_string()).unwrap();
        let mut parser = AstParser::new(tokens.clone());
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Preamble::create(
                PreambleType::LetIn(PreambleLetIn {
                    expr: vec![Binary::create(
                        Literal::create(tokens[1].clone()),
                        tokens[2].clone(),
                        Literal::create(tokens[3].clone())
                    )]
                }),
                Binary::create(
                    Literal::create(tokens[6].clone()),
                    tokens[7].clone(),
                    Literal::create(tokens[8].clone())
                )
            )
        )
    }

    #[test]
    pub fn with_primary() {
        let tokens = parse_text("with a; b".to_string()).unwrap();

        let mut parser = AstParser::new(tokens.clone());
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Preamble::create(
                PreambleType::With(PreambleWith {
                    expr: Box::new(Literal::create(tokens[1].clone()))
                }),
                Literal::create(tokens[3].clone())
            )
        )
    }
}
