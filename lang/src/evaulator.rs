use std::collections::HashMap;

use crate::ast::{Expr, SpannedExpr};
use crate::lexer::Span;

#[derive(Debug)]
pub struct EvalError {
    pub message: String,
    pub message_short: String,
    pub span: Span,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn eval<'src>(expr: &'src SpannedExpr, context: &mut HashMap<String, Expr>) -> Result<Expr, EvalError> {    
    match &expr.node {
        Expr::Int(n) => Ok(Expr::Int(*n)),
        Expr::Float(f) => Ok(Expr::Float(*f)),
        Expr::String(s) => Ok(Expr::String(s.clone())),
        Expr::Bool(b) => Ok(Expr::Bool(*b)),
        Expr::Null => Ok(Expr::Null),

        Expr::Identifier(name) => {
            match context.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(EvalError {
                    message: format!("Undefined variable: {}", name),
                    message_short: "not defined".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Call { name, args } => {
            let evaluated_args: Result<Vec<SpannedExpr>, EvalError> = args.iter()
                .map(|arg| {
                    match eval(arg, context) {
                        Ok(v) => Ok(SpannedExpr {
                            node: v,
                            span: arg.span,
                        }),
                        Err(e) => Err(e),
                    }
                })
                .collect();

            match context.get(name) {
                Some(v) => {
                    match v {
                        Expr::InternalFunction { name, args, func } => {
                            if !args.contains(&"__args__".to_string()) && args.len() != evaluated_args.as_ref().map_or(0, |a| a.len()) {
                                return Err(EvalError {
                                    message: format!("Function {} expects {} arguments, got {}", name, args.len(), evaluated_args.as_ref().map_or(0, |a| a.len())),
                                    message_short: format!("got {} arguments too many", evaluated_args.as_ref().map_or(0, |a| a.len()) - args.len()),
                                    span: expr.span,
                                });
                            }

                            match func(evaluated_args?) {
                                Ok(response) => Ok(response.return_value.node),
                                Err((msg, span)) => Err(EvalError {
                                    message: msg.clone(),
                                    message_short: msg,
                                    span,
                                }),
                            }
                        }

                        _ => Err(EvalError {
                            message: format!("{} is not a function", name),
                            message_short: "not a function".to_string(),
                            span: expr.span,
                        })
                    }
                }

                None => Err(EvalError {
                    message: format!("Undefined function: {}", name),
                    message_short: "not defined".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Let { name, value } => {
            context.insert(name.clone(), (*value).node.clone());

            Ok(Expr::Null)
        }

        v => {
            Err(EvalError {
                message: format!("Cannot evaluate expression: {:?}", v),
                message_short: "cannot evaluate".to_string(),
                span: expr.span,
            })
        }
    }
}