use crate::{
    ast::{
        parser::AstParser,
        types::{Assignment, Binary, Literal},
    },
    lexer::parse_text,
};

#[test]
pub fn assignment() {
    let tokens = parse_text("a = 2;").unwrap();

    let mut parser = AstParser::new(tokens.clone());

    let ast = parser.parse().unwrap();

    assert_eq!(
        ast,
        Assignment::new_expr(tokens[0].clone(), Literal::new_expr(tokens[2].clone()))
    )
}

#[test]
pub fn assignment_and_arithmetic() {
    let tokens = parse_text("a = 1+2+3;").unwrap();

    let mut parser = AstParser::new(tokens.clone());

    let ast = parser.parse().unwrap();

    assert_eq!(
        ast,
        Assignment::new_expr(
            tokens[0].clone(),
            Binary::new(
                Binary::new(
                    Literal::new_expr(tokens[2].clone()),
                    tokens[3].clone(),
                    Literal::new_expr(tokens[4].clone())
                ),
                tokens[5].clone(),
                Literal::new_expr(tokens[6].clone())
            )
        )
    )
}
