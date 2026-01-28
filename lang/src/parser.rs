use std::collections::HashMap;

use chumsky::prelude::*;
use crate::ast::Expr;

fn parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>> {
    
}

pub fn parse(input: &str, context: &mut HashMap<String, Expr>) {
    match parser().parse(input).into_result() {
        Ok(expr) => {
            println!("Parsed expression: {:?}", expr);
        }

        Err(errors) => {
            for e in errors {
                println!("Parse error: {}", e);
            }
        }
    }
}