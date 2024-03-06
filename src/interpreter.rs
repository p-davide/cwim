use crate::env::Env;
use crate::function::*;
use crate::parser::*;
use crate::pratt;
use crate::s;
use crate::s::polynomial;
use crate::s::Polynomial;
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
            if let Some(p) = polynomial(&expr, env) {
                let result = s::eval(&pratt::expr(&mut rhs, env));
                let p2 = p.add(&Polynomial::new("", -result));
                match env.assign(
                    p.unknown.to_owned(),
                    p2.zeros()[0], // TODO: Allow multiple solutions to be assigned.
                ) {
                    Some(_) => Err(format!("{} already exists", p.unknown)),
                    None => Ok(result),
                }
            } else {
                Err("invalid polynomial".to_owned())
            }
        }
    }
}
