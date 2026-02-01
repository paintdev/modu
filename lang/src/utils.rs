use std::collections::HashMap;

pub fn create_context() -> HashMap<String, crate::ast::Expr> {
    let mut context = HashMap::new();

    crate::builtins::misc::fill_context(&mut context);

    return context;
}