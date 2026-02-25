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
}
