use crate::function::{Function, F};
use std::fmt;

use crate::number::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum S<'a> {
    Var(Number),
    Fun(Function<'a>, Vec<S<'a>>),
    Unknown(&'a str),
}

impl<'a> fmt::Display for S<'a> {
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
            S::Unknown(x) => write!(f, "{}", x),
        }
    }
}

pub fn eval(s: &S) -> Number {
    match s {
        S::Var(n) => n.clone(),
        S::Fun(f, xs) => match f.f {
            F::Nary(f) => f(Number {
                inner: xs.iter().rev().map(|s| eval(s).inner[0]).collect(),
            }),
            F::Binary(f) => {
                if let Some(n) = xs
                    .iter()
                    .rev()
                    .map(|s| Number::scalar(eval(s).inner[0]))
                    .reduce(f)
                {
                    n
                } else {
                    Number { inner: vec![] }
                }
            }
        },
        S::Unknown(x) => panic!("tried to evaluate unknown {}", x),
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
                S::Var(Number::scalar(2.)),
                S::Fun(
                    ADD,
                    vec![
                        S::Fun(ID, vec![S::Var(Number::scalar(3.))]),
                        S::Var(Number::scalar(5.)),
                    ],
                ),
            ],
        );
        let stmt = parser::parse("2(+3+5)", &env::Env::prelude()).unwrap();
        match stmt {
            Stmt::Expr(mut tokens) => {
                let it = env::Env::prelude();
                let actual = expr(&mut tokens, &it).unwrap();
                assert_eq!(expected, actual);
                assert_eq!(eval(&expected), eval(&actual));
            }
            _ => panic!(),
        }
    }
}
