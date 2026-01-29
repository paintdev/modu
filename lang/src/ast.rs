use crate::lexer::Span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct InternalFunctionResponse {
    pub return_value: Spanned<Expr>,
    pub replace_self: Option<Spanned<Expr>>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Bool(bool),
    Null,

    Call {
        name: String,
        args: Vec<Spanned<Expr>>,
    },

    Let {
        name: String,
        value: Box<Spanned<Expr>>,
    },

    InternalFunction {
        name: String,
        args: Vec<Spanned<Expr>>, // or __args__ for an optional amount
        func: fn(Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, Span)>,
    }
}

pub type SpannedExpr = Spanned<Expr>;