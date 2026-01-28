#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Var(String),

    Call(String, Vec<Expr>),

    Let {
        name: String,
        value: Box<Expr>,
    }
}