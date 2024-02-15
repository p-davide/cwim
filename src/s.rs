use crate::function::Function;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum S {
    Var(f64),
    Fun(Function, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Var(i) => write!(f, "{}", i),
            S::Fun(head, rest) => {
                write!(f, "({}", head.name)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}

pub fn eval(s: &S) -> f64 {
    match s {
        S::Var(n) => *n,
        S::Fun(f, xs) => (f.f)(xs.iter().rev().map(eval).collect()),
    }
}

#[cfg(test)]
mod test {
    use crate::{
        env,
        function::{ADD, ID, MUL},
        parser::{self, Stmt},
        pratt::expr,
    };

    use super::*;

    #[test]
    fn _a() {
        let expected = S::Fun(
            MUL,
            vec![
                S::Var(2.),
                S::Fun(ADD, vec![S::Fun(ID, vec![S::Var(3.)]), S::Var(5.)]),
            ],
        );
        let stmt = parser::parse("2(+3+5)", &env::Env::prelude()).unwrap();
        match stmt {
            Stmt::Expr(mut tokens) => {
                let actual = expr(&mut tokens, &mut env::Env::prelude());
                assert_eq!(expected, actual);
                assert_eq!(eval(&expected), eval(&actual));
            }
            _ => panic!(),
        }
    }
}
