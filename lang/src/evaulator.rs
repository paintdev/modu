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

        Expr::Neg(inner) => {
            let value = eval(inner, context)?;

            match value {
                Expr::Int(n) => Ok(Expr::Int(-n)),
                Expr::Float(f) => Ok(Expr::Float(-f)),
                _ => Err(EvalError {
                    message: format!("Cannot negate value: {:?}", value),
                    message_short: "cannot negate".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Add(left, right) => {
            let left_value = eval(left, context)?;
            let right_value = eval(right, context)?;

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Expr::Int(l + r)),
                (Expr::Float(l), Expr::Float(r)) => Ok(Expr::Float(l + r)),
                (Expr::Int(l), Expr::Float(r)) => Ok(Expr::Float(l as f64 + r)),
                (Expr::Float(l), Expr::Int(r)) => Ok(Expr::Float(l + r as f64)),
                (Expr::String(l), Expr::String(r)) => Ok(Expr::String(l + &r)),

                _ => Err(EvalError {
                    message: format!("Cannot add values: {:?} + {:?}", left.node, right.node),
                    message_short: "cannot add".to_string(),
                    span: expr.span,
                }),
            }
        }

        Expr::Sub(left, right) => {
            let left_value = eval(left, context)?;
            let right_value = eval(right, context)?;

            match (left_value, right_value) {
                (Expr::Int(l), Expr::Int(r)) => Ok(Expr::Int(l - r)),
                (Expr::Float(l), Expr::Float(r)) => Ok(Expr::Float(l - r)),
                (Expr::Int(l), Expr::Float(r)) => Ok(Expr::Float(l as f64 - r)),
                (Expr::Float(l), Expr::Int(r)) => Ok(Expr::Float(l - r as f64)),
                
                _ => Err(EvalError {
                    message: format!("Cannot subtract values: {:?} - {:?}", left.node, right.node),
                    message_short: "cannot subtract".to_string(),
                    span: expr.span,
                }),
            }
        }

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
            match eval(value, context) {
                Ok(v) => {
                    context.insert(name.clone(), v);
                    
                    Ok(Expr::Null)
                }

                Err(e) => Err(e),
            }
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