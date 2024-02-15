use crate::env::Env;
use crate::function::*;
use crate::parser::*;
use crate::pratt;
use crate::s;
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
        Stmt::Assignment(name, mut expr) => {
            let result = s::eval(&pratt::expr(&mut expr, env));
            match env.assign(name, result) {
                Some(_) => Err("already exists".to_owned()),
                None => Ok(result),
            }
        }
    }
}
