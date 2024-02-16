use crate::function::Function;
use std::{collections::HashMap, fmt, ops};

#[derive(Clone, Debug, PartialEq)]
pub enum S<N> {
    Var(N),
    Fun(Function<N>, Vec<S<N>>),
}

pub struct Polynomial {
    coefs: Vec<f64>,
}

impl ops::Add for Polynomial {
    type Output = Polynomial;

    fn add(self, rhs: Polynomial) -> Self::Output {
        let mut result = vec![];
        let zero = 0f64;
        for (i, j) in self
            .coefs
            .iter()
            .rev()
            .chain(std::iter::once(&zero).cycle())
            .zip(rhs.coefs.iter().rev().chain(std::iter::once(&zero).cycle()))
            .take(std::cmp::max(self.coefs.len(), rhs.coefs.len()))
        {
            result.push(i + j);
        }
        result.reverse();
        Self { coefs: result }
    }
}

impl ops::Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Polynomial) -> Self::Output {
        let deg = self.degree() + rhs.degree() - 1;
        let mut result = vec![0f64; deg];
        let mut hm: HashMap<(usize, usize), f64> = HashMap::new();
        for (i, x) in self.coefs.iter().rev().enumerate() {
            for (j, y) in rhs.coefs.iter().rev().enumerate() {
                hm.insert((i, j), x * y);
            }
        }
        for deg in 0..self.coefs.len() + rhs.coefs.len() - 1 {
            let mut coef = 0f64;
            for (_, b) in hm.iter().filter(|(k, _)| k.0 + k.1 == deg) {
                coef += *b;
            }
            result.push(coef)
        }
        Self { coefs: result }
    }
}

impl ops::Neg for Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Self::Output {
        Polynomial {
            coefs: self.coefs.iter().map(|x| -x).collect(),
        }
    }
}

impl Polynomial {
    pub fn of(x: f64) -> Self {
        Self { coefs: vec![x] }
    }
    pub fn eval(&self, x: f64) -> f64 {
        let mut result = 0f64;
        for (exp, coef) in self.coefs.iter().rev().enumerate() {
            result += coef * x.powi(exp.try_into().unwrap());
        }
        result
    }
    pub fn degree(&self) -> usize {
        self.coefs.iter().skip_while(|it| **it == 0.).count()
    }
}

impl<N: fmt::Display> fmt::Display for S<N> {
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

pub fn eval<N: Clone>(s: &S<N>) -> N {
    match s {
        S::Var(n) => n.clone(),
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
            MUL(),
            vec![
                S::Var(2.),
                S::Fun(ADD(), vec![S::Fun(ID(), vec![S::Var(3.)]), S::Var(5.)]),
            ],
        );
        let stmt = parser::parse::<f64>("2(+3+5)", &env::Env::prelude()).unwrap();
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
