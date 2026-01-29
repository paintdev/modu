use std::collections::HashMap;

pub fn is_reserved(name: &str) -> bool {
    match name {
        "let" | "fn" | "import" | "if" | "null" | "return" | "as" | "loop" => true,
        _ => false,
    }
}

pub fn create_context() -> HashMap<String, crate::ast::Expr> {
    let mut context = HashMap::new();

    crate::functions::misc::fill_context(&mut context);
    crate::functions::type_conversion::fill_context(&mut context);

    return context;
}