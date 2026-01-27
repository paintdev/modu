use std::collections::HashMap;

use crate::ast::AST;
use crate::eval::eval;

fn encode(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("base64.encode() expects a string argument".to_string()),
    };

    let encoded = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD, input
    );

    Ok((AST::String(encoded), AST::Null))
}

fn decode(args: Vec<AST>, context: &mut HashMap<String, AST>) -> Result<(AST, AST), String> {
    let input = match eval(args[0].clone(), context)? {
        AST::String(str) => str,
        _ => return Err("base64.decode() expects a string argument".to_string()),
    };

    let decoded = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD, input
    )
        .map_err(|e| format!("base64.decode() failed: {}", e))?
        .iter()
        .map(|&c| c as char)
        .collect::<String>();

    Ok((AST::String(decoded), AST::Null))
}

pub fn get_object() -> HashMap<String, AST> {
    let mut object = HashMap::new();

    object.insert(
        "encode".to_string(),
        AST::InternalFunction { 
            name: "encode".to_string(), 
            args: vec!["str".to_string()], 
            call_fn: encode 
        }
    );

    object.insert(
        "decode".to_string(),
        AST::InternalFunction { 
            name: "decode".to_string(), 
            args: vec!["str".to_string()], 
            call_fn: decode 
        }
    );

    return object;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_time_package() {
        let object = get_object();

        assert_eq!(object.len(), 2);
    }
}