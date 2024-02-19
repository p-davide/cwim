use num_traits::real::Real;

use crate::env::Env;
use crate::function::*;
use crate::parser::*;
use crate::pratt;
use crate::s;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
pub enum Expr<N> {
    Literal(N),
    Function(Function<N>),
    Variable(String, N),
}

impl<N: std::fmt::Debug> Debug for Expr<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "{:?}", n),
            Expr::Function(g) => write!(f, "{:?}", g),
            Expr::Variable(name, coef) => write!(f, "V{:?}{:?}", coef, name),
        }
    }
}

pub fn run<N: Real + Debug + FromStr>(text: &str, env: &mut Env<N>) -> Parsed<N>
where
    <N as FromStr>::Err: std::fmt::Debug,
{
    let tks = parse(text, env)?;
    match tks {
        Stmt::Expr(mut tks) => Ok(s::eval(&pratt::Lexer{lexer: &mut tks, env}.expr())),
        Stmt::Assignment(name, mut tks) => {
            let result = s::eval(&pratt::Lexer{lexer: &mut tks, env}.expr());
            env.assign(name, result);
            Ok(result)
        }
    }
}
