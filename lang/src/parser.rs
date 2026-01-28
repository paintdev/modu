use std::collections::HashMap;
use chumsky::prelude::*;

use crate::{ast::Expr, lexer::{Token, lex}};

fn parser<'src>() -> impl Parser<'src, &'src [Token], Vec<Expr>, extra::Err<Rich<'src, Token>>> {
    let expr = select! {
        Token::Int(i) => Expr::Int(i),
    };

    let let_stmt = just(Token::Let)
        .ignore_then(select! { Token::Identifier(name) => name })
        .then_ignore(just(Token::Equals))
        .then(expr)
        .then_ignore(just(Token::Semicolon))
        .map(|(name, value)| Expr::Let {
            name,
            value: Box::new(value),
        });
    
    let_stmt.repeated().collect().then_ignore(end())
}

pub fn parse(input: &str, context: &mut HashMap<String, Expr>) {
    let tokens = match lex(input) {
        Ok(toks) => toks,
        Err(e) => {
            println!("Lexing error: {:?}", e);
            return;
        }
    };

    match parser().parse(&tokens).into_result() {
        Ok(ast) => {
            println!("Parsed AST: {:?}", ast);
        }
        Err(e) => {
            println!("Parsing error: {:?}", e);
        }
    }
}