mod time;

pub fn get_package(name: &str) -> Option<crate::ast::Expr> {
    match name {
        "time" => Some(time::get_object()),
        _ => None,
    }
}