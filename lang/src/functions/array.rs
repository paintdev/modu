use crate::ast::AST;

pub fn clear(args: Vec<AST>) -> Result<Vec<AST>, String> {
    if args.len() != 0 {
        return Err("clear() takes no arguments".to_string());
    }

    Ok(vec![])
}

pub fn push(
    elements: &Vec<AST>,
    args: Vec<AST>,
) -> Result<Vec<AST>, String> {
    if args.len() != 1 {
        return Err("push() takes exactly one argument".to_string());
    }

    let mut new_elements = elements.clone();
    new_elements.push(args[0].clone());

    Ok(new_elements)
}

pub fn handle_function(
    elements: &Vec<AST>,
    function: String,
    args: Vec<AST>,
) -> Result<(AST, AST), String> {
    match function.as_str() {
        "clear" => {
            let result = clear(args)?;
            Ok((AST::Null, AST::Array(result)))
        }

        "push" => {
            let result = push(elements, args)?;
            Ok((AST::Null, AST::Array(result)))
        }
        
        _ => Err(format!("Function '{}' not found in array functions", function)),
    }
}