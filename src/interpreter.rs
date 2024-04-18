use crate::env::Env;
use crate::function::*;
use crate::number::Number;
use crate::parser::*;
use crate::polynomial::polynomial;
use crate::pratt;
use crate::s;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone, PartialEq)]
pub enum Expr<'f> {
    Literal(Number),
    Function(Function<'f>),
    Variable(String, Number),
}

impl<'f> Debug for Expr<'f> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "{:?}", n),
            Expr::Function(g) => write!(f, "{:?}", g),
            Expr::Variable(name, coef) => write!(f, "V{:?}{:?}", coef, name),
        }
    }
}

pub fn run(text: &str, env: &mut Env) -> Parsed<Number> {
    let tks = parse(text, env)?;
    match tks {
        Stmt::Expr(mut tks) => Ok(s::eval(&pratt::expr(&mut tks, env)?)),
        Stmt::Assignment(mut lhs, mut rhs) => {
            let expr = pratt::expr(&mut lhs, env)?;
            let mut p = polynomial(&expr, env)?;
            // example: in x^2 + 2x = 6+5, result = 11
            let result = s::eval(&pratt::expr(&mut rhs, env)?);
            p += -&result;
            // TODO: Allow multiple solutions to be assigned.
            let zeros = p.zeros();
            if !zeros.inner.is_empty() {
                match env.assign(p.unknown.to_owned(), &zeros) {
                    Some(_) => Err("Variable already exists".to_owned()),
                    None => Ok(zeros),
                }
            } else {
                return Err("no solution found".to_owned());
            }
        }
    }
}
