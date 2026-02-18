use crate::parser::token::{
    Token,
    types::{LiteralToken, MathToken, TokenType},
};

trait Visitor<TY> {
    fn visit_binary<L: Expr, R: Expr>(&self, b: &Binary<L, R>) -> TY;
    fn visit_grouping<E: Expr>(&self, g: &Grouping<E>) -> TY;
    fn visit_unary<R: Expr>(&self, u: &Unary<R>) -> TY;
    fn visit_literal(&self, l: &Literal) -> TY;
}

pub trait Expr {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY;
}

pub struct Binary<L, R>
where
    L: Expr,
    R: Expr,
{
    left: L,
    operator: Token,
    right: R,
}

impl<L: Expr, R: Expr> Expr for Binary<L, R> {
    fn accept<E, T: Visitor<E>>(&self, visitor: &T) -> E {
        visitor.visit_binary(self)
    }
}

pub struct Grouping<E>
where
    E: Expr,
{
    expr: E,
}

impl<E: Expr> Expr for Grouping<E> {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_grouping(self)
    }
}

pub struct Literal {
    literal: Token,
}

impl Expr for Literal {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_literal(self)
    }
}

pub struct Unary<R>
where
    R: Expr,
{
    operator: Token,
    right: R,
}

impl<R: Expr> Expr for Unary<R> {
    fn accept<TY, T: Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_unary(self)
    }
}

struct AstPrinter {}
impl AstPrinter {
    fn print(expr: impl Expr) -> String {
        let printer = AstPrinter {};

        expr.accept(&printer)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary<L: Expr, R: Expr>(&self, b: &Binary<L, R>) -> String {
        format!(
            "({} {} {})",
            b.operator.unparsed,
            b.left.accept(self),
            b.right.accept(self)
        )
    }

    fn visit_grouping<E: Expr>(&self, g: &Grouping<E>) -> String {
        format!("(group {})", g.expr.accept(self))
    }

    fn visit_unary<R: Expr>(&self, u: &Unary<R>) -> String {
        format!("({} {})", u.operator.unparsed, u.right.accept(self))
    }

    fn visit_literal(&self, l: &Literal) -> String {
        l.literal.unparsed.clone()
    }
}

pub fn ast_test() {
    let expr: impl Expr = Binary {
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
