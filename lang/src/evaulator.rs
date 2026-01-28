use chumsky::prelude::*;

use crate::ast::Expr;

fn eval<'src>(expr: &'src Expr) -> Result<Expr, String> {
    match expr {
        _ => {
            todo!()
        }
    }
}