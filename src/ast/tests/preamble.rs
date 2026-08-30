#[cfg(test)]
mod test {
    use crate::{
        ast::{
            parser::AstParser,
            types::{Binary, Literal, Preamble, PreambleLetIn, PreambleType, PreambleWith},
        },
        lexer::parse_text,
    };

    #[test]
    pub fn let_in_primary() {
        let tokens = parse_text("let a = 2; in 1").unwrap();

        let mut parser = AstParser::new(tokens.clone());

        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Preamble::new(
                PreambleType::LetIn(PreambleLetIn {
                    expr: vec![Binary::new(
                        Literal::new_expr(tokens[1].clone()),
                        tokens[2].clone(),
                        Literal::new_expr(tokens[3].clone())
                    )]
                }),
                Literal::new_expr(tokens[6].clone())
            )
        )
    }

    #[test]
    pub fn let_in_assignment() {
        let tokens = parse_text("let a = 2; in b = a;").unwrap();
        let mut parser = AstParser::new(tokens.clone());
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Preamble::new(
                PreambleType::LetIn(PreambleLetIn {
                    expr: vec![Binary::new(
                        Literal::new_expr(tokens[1].clone()),
                        tokens[2].clone(),
                        Literal::new_expr(tokens[3].clone())
                    )]
                }),
                Binary::new(
                    Literal::new_expr(tokens[6].clone()),
                    tokens[7].clone(),
                    Literal::new_expr(tokens[8].clone())
                )
            )
        )
    }

    #[test]
    pub fn with_identifier() {
        let tokens = parse_text("with a; b").unwrap();

        let mut parser = AstParser::new(tokens.clone());
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Preamble::new(
                PreambleType::With(PreambleWith {
                    expr: Box::new(Literal::new_expr(tokens[1].clone()))
                }),
                Literal::new_expr(tokens[3].clone())
            )
        )
    }

    #[test]
    pub fn with_primary() {
        let tokens = parse_text("with a; 1 + b").unwrap();

        let mut parser = AstParser::new(tokens.clone());
        let ast = parser.parse().unwrap();

        assert_eq!(
            ast,
            Preamble::new(
                PreambleType::With(PreambleWith {
                    expr: Box::new(Literal::new_expr(tokens[1].clone()))
                }),
                Binary::new(
                    Literal::new_expr(tokens[3].clone()),
                    tokens[4].clone(),
                    Literal::new_expr(tokens[5].clone())
                )
            )
        )
    }
}
