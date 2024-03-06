use crate::env::Env;
use crate::function::*;
use crate::parser::*;
use crate::pratt;
use crate::s;
use crate::s::polynomial;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Literal(f64),
    Function(Function),
    Variable(String, f64),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "{:?}", n),
            Expr::Function(g) => write!(f, "{:?}", g),
            Expr::Variable(name, coef) => write!(f, "V{:?}{:?}", coef, name),
        }
    }
}

pub fn run(text: &str, env: &mut Env) -> Parsed<f64> {
    let tks = parse(text, env)?;
    match tks {
        Stmt::Expr(mut tks) => Ok(s::eval(&pratt::expr(&mut tks, env))),
        Stmt::Assignment(mut lhs, mut rhs) => {
            let expr = pratt::expr(&mut lhs, env);
            if let Some(mut p) = polynomial(&expr, env) {
                // example: in x^2 + 2x = 6+5, result = 11
                let result = s::eval(&pratt::expr(&mut rhs, env));
                p += -result;
                // TODO: Allow multiple solutions to be assigned.
                if let Some(zero) = p.zeros().pop() {
                    match env.assign(p.unknown.to_owned(), zero) {
                        Some(_) => Err(format!("{} already exists", p.unknown)),
                        None => Ok(zero),
                    }
                } else {
                    return Err("no solution found".to_owned());
                }
            } else {
                Err("invalid polynomial".to_owned())
            }
        }
    }
}
