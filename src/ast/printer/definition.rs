use itertools::Itertools;

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

    fn visit_preamble(&self, p: &crate::ast::types::Preamble) -> String {
        format!("(preamble {})", p.expr.accept(self))
    }

    fn visit_list(&self, l: &crate::ast::types::List) -> String {
        format!("[ {} ]", l.list.iter().map(|e| e.accept(self)).join("  "))
    }

    fn visit_function(&self, f: &crate::ast::types::Function) -> String {
        format!(
            "(function '{}': {} {})",
            f.identifier.unparsed,
            f.preamble
                .as_ref()
                .map(|preamble| self.visit_preamble(preamble))
                .unwrap_or_default(),
            f.expr.accept(self)
        )
    }

    fn visit_assignment(&self, a: &crate::ast::types::Assignment) -> String {
        format!(
            "(assignment '{}' = '{}')",
            a.identifier.unparsed,
            a.expr.accept(self)
        )
    }

    fn visit_attrset(&self, a: &crate::ast::types::Attrset) -> String {
        format!(
            "(attrset {})",
            a.values.iter().map(|elem| elem.accept(self)).join(", ")
        )
    }

    fn visit_inherit(&self, i: &crate::ast::types::inherit::Inherit) -> String {
        format!(
            "(inherit ({}) {})",
            match &i.inherit_from {
                Some(value) => value.accept(self),
                None => "".to_string(),
            },
            i.inherit_values.iter().map(|e| &e.unparsed).join(", ")
        )
    }
}
