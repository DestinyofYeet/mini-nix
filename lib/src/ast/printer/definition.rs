use crate::ast::types::{Binary, Expr, Grouping, Literal, Unary, Visitor};

pub struct AstPrinter {}
impl AstPrinter {
    pub fn print(expr: impl Expr) -> String {
        let printer = AstPrinter {};

        expr.accept(&printer)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(&self, b: &Binary) -> String {
        format!(
            "({} {} {})",
            b.operator.unparsed,
            b.left.accept(self),
            b.right.accept(self)
        )
    }

    fn visit_grouping(&self, g: &Grouping) -> String {
        format!("(group {})", g.expr.accept(self))
    }

    fn visit_unary(&self, u: &Unary) -> String {
        format!("({} {})", u.operator.unparsed, u.right.accept(self))
    }

    fn visit_literal(&self, l: &Literal) -> String {
        l.literal.unparsed.clone()
    }
}
