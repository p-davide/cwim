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
    let tks = stmt(text, env)?;
    match tks {
        Stmt::Expr(mut tks) => Ok(s::eval(&pratt::expr(&mut tks, env)?)?),
        Stmt::Assignment(mut lhs, mut rhs) => {
            let expr = pratt::expr(&mut lhs, env)?;
            let mut p = polynomial(&expr, env)?;
            // example: in x^2 + 2x = 6+5, result = 11
            let result = s::eval(&pratt::expr(&mut rhs, env)?)?;
            p -= result;
            // TODO: Allow multiple solutions to be assigned.
            let roots = p.roots();
            match roots[..] {
                [root] => {
                    env.assign(p.unknown.to_owned(), &root);
                    Ok(root)
                }
                [root1, root2] => {
                    println!("{}, {}", root1, root2);
                    env.assign(p.unknown.to_owned(), &root1);
                    Ok(root1)
                }
                _ => {
                    return Err("no solution found".to_owned());
                }
            }
        }
    }
}
