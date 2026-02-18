use crate::{
    ast::{
        printer::AstPrinter,
        types::{Binary, Expr, Grouping, Literal, Unary},
    },
    lexer::token::{
        Token,
        types::{LiteralToken, MathToken, TokenType},
    },
};

pub fn ast_test() {
    let expr = Binary {
        left: Unary {
            operator: Token::new(TokenType::Math(MathToken::Minus), "-", 1),
            right: Literal {
                literal: Token::new(TokenType::Literal(LiteralToken::Integer(123)), "123", 1),
            },
        },
        operator: Token::new(TokenType::Math(MathToken::Star), "*", 1),
        right: Grouping {
            expr: Literal {
                literal: Token::new(TokenType::Literal(LiteralToken::Float(45.67)), "45.67", 1),
            },
        },
    };

    println!("{}", AstPrinter::print(expr));
}
