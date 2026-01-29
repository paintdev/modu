use crate::{ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr}, lexer::Span};

pub fn print(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)> {
    for arg in args {
        print!("{}", arg.node);
    }
    println!();

    Ok(InternalFunctionResponse {
        return_value: SpannedExpr {
            node: Expr::Null,
            span: Span::default(),
        },
        replace_self: None,
    })
}