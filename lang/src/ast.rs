
use std::collections::HashMap;
use libloading::Library;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    Int(i64),
    Float(f64),
    Var(&'src str),

    Call(&'src str, Vec<Expr<'src>>),

    Let {
        name: &'src str,
        value: Box<Expr<'src>>,
    }
}