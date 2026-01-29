use std::collections::HashMap;

pub fn is_reserved(name: &str) -> bool {
    match name {
        "let" | "fn" | "import" | "if" | "null" | "return" | "as" | "loop" => true,
        _ => false,
    }
}

pub fn create_context() -> HashMap<String, crate::ast::Expr> {
    let mut context = HashMap::new();

    context.insert(
        "print".to_string(),
        crate::ast::Expr::InternalFunction {
            name: "print".to_string(),
            args: vec!["__args__".to_string()],
            func: crate::functions::misc::print,
        },
    );

    return context;
}