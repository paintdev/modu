use std::process::Command;
use crate::ast::{Expr, InternalFunctionResponse, Spanned, SpannedExpr};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

fn clean_command(cmd: String) -> String {
	let clean = cmd.trim()
		.trim_matches('"')
		.trim_matches('\'')
		.to_string();

	return clean;
}

pub fn exec(args: Vec<Spanned<Expr>>) -> Result<InternalFunctionResponse, (String, crate::lexer::Span)> {
    if args.len() != 1 {
        return Err((
            "exec takes exactly one argument".to_string(),
            args[1].span,
        ));
    }

    let command_str = match &args[0].node {
        Expr::String(s) => clean_command(s.clone()),
        _ => return Err((
            "exec expects a string argument".to_string(),
            args[0].span,
        )),
    };

    let output = {
        #[cfg(windows)]
        {
            Command::new("cmd")
                .args(&["/C", &command_str])
                .creation_flags(0x08000000)
                .output()
        }

        #[cfg(not(windows))]
        {
            Command::new("sh")
                .arg("-c")
                .arg(&command_str)
                .output()
        }
    }.map_err(|e| (
        format!("Failed to execute command: {}", e),
        args[0].span,
    ))?;
}

pub fn get_object() -> Expr {
    let mut symbols = std::collections::HashMap::new();

    symbols.insert(
        "exec".to_string(),
        SpannedExpr {
            node: Expr::InternalFunction {
                name: "exec".to_string(),
                args: vec![],
                func: exec,
            },
            span: crate::lexer::Span::default(),
        },
    );

    Expr::Module { symbols }
}