use crate::{
    function::{Function, F},
    parser::Parsed,
};
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

pub fn eval(s: &S) -> Parsed<Number> {
    match s {
        S::Var(n) => Ok(n.clone()),
        S::Fun(fun, ss) => match fun.f {
            F::Nary(f) => Ok(f(eval(&ss[0])?)),
            F::Binary(f) => {
                let mut result = None;
                for s in ss {
                    let next = eval(s)?;
                    result = match result {
                        None => Some(next),
                        Some(curr) => Some(f(next, curr)),
                    }
                }
                match result {
                    None => Err(format!(
                        "Binary function {} was called with no arguments",
                        fun.name
                    )),
                    Some(n) => Ok(n),
                }
            }
        },
        S::Unknown(x) => Err(format!("tried to evaluate unknown {}", x)),
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
                S::Var(Number::Int(2)),
                S::Fun(
                    ADD,
                    vec![
                        S::Fun(ID, vec![S::Var(Number::Int(3))]),
                        S::Var(Number::Int(5)),
                    ],
                ),
            ],
        );
        let stmt = parser::stmt("2(+3+5)", &env::Env::prelude()).unwrap();
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
