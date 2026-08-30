use crate::ast::types::{Expr, inherit::Inherit};

impl Expr for Inherit {
    fn accept<TY, T: crate::ast::types::Visitor<TY>>(&self, visitor: &T) -> TY {
        visitor.visit_inherit(self)
    }
}
