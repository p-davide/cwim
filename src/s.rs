use crate::{
    env,
    function::{Function, ADD, MUL, POW, SUB},
};
use std::{
    fmt,
    iter::{repeat, zip},
    ops,
};

#[derive(Clone, Debug, PartialEq)]
pub enum S<'a> {
    Var(f64),
    Fun(Function, Vec<S<'a>>),
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

pub fn eval(s: &S) -> f64 {
    match s {
        S::Var(n) => *n,
        S::Fun(f, xs) => (f.f)(xs.iter().rev().map(eval).collect()),
        S::Unknown(x) => panic!("tried to evaluate unknown {}", x),
    }
}

#[derive(Debug, Clone)]
pub struct Polynomial<'a> {
    pub unknown: &'a str,
    coefs: Vec<f64>,
}

// The 1st element gives the 0th grade coefficient
impl<'a> Polynomial<'a> {
    pub fn new(unknown: &'a str, n: f64) -> Self {
        Self {
            unknown,
            coefs: vec![n],
        }
    }
    fn set_unknown(&mut self, name: &'a str) {
        self.unknown = if self.unknown == name || name == "" {
            self.unknown
        } else if self.unknown == "" {
            name
        } else {
            panic!("two different unknowns: {}, {}", self.unknown, name)
        };
    }
    pub fn zeros(&self) -> Vec<f64> {
        match &self.coefs[..] {
            &[] => panic!("empty polynomial"),
            &[_] => vec![],
            &[b, a] => vec![-b / a],
            &[c, b, a] => {
                let delta = (b * b - 4. * a * c).sqrt();
                // TODO: complex solutions
                if delta.is_nan() {
                    return vec![];
                }
                vec![(-b - delta) / (2. * a), (-b + delta) / (2. * a)]
            }
            // TODO: higher order polynomials
            _ => vec![],
        }
    }
}

impl<'a> ops::Add<Self> for &Polynomial<'a> {
    type Output = Polynomial<'a>;
    fn add(self, other: Self) -> Self::Output {
        let (longest, shortest) = if self.coefs.len() > other.coefs.len() {
            (&(self.coefs), &(other.coefs))
        } else {
            (&(other.coefs), &(self.coefs))
        };
        let coefs = zip(longest.iter(), shortest.iter().chain(repeat(&0.)))
            .map(|(x, y)| x + y)
            .collect();
        let mut result = Polynomial { coefs, ..*self };
        result.set_unknown(other.unknown);
        result
    }
}

impl<'a> ops::AddAssign<&Self> for Polynomial<'a> {
    fn add_assign(&mut self, rhs: &Self) {
        let result = &*self + rhs;
        self.coefs = result.coefs;
        self.unknown = result.unknown;
    }
}

impl<'a> ops::Mul<Self> for &Polynomial<'a> {
    type Output = Polynomial<'a>;
    fn mul(self, other: Self) -> Self::Output {
        let mut result = vec![0_f64; self.coefs.len() + other.coefs.len() - 1];
        for (d1, c1) in self.coefs.iter().enumerate() {
            for (d2, c2) in other.coefs.iter().enumerate() {
                result[d1 + d2] += c1 * c2;
            }
        }
        let mut result = Polynomial {
            coefs: result,
            ..*self
        };
        result.set_unknown(other.unknown);
        result
    }
}

impl<'a> ops::MulAssign<&Self> for Polynomial<'a> {
    fn mul_assign(&mut self, rhs: &Self) {
        let result = &*self * &rhs;
        self.coefs = result.coefs;
        self.unknown = result.unknown;
    }
}

impl<'a> ops::Add<f64> for Polynomial<'a> {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<'a> ops::AddAssign<f64> for Polynomial<'a> {
    fn add_assign(&mut self, other: f64) {
        self.coefs[0] += other;
    }
}

impl<'a> ops::Neg for Polynomial<'a> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            coefs: self.coefs.iter().map(|x| -x).collect(),
            ..self
        }
    }
}

pub fn polynomial<'a>(s: &S<'a>, env: &env::Env) -> Option<Polynomial<'a>> {
    match s {
        S::Var(n) => Some(Polynomial::new("", *n)),
        S::Fun(f, ss) => {
            if f == &ADD {
                let mut result = Polynomial::new("", 0.);
                for s in ss {
                    result += &polynomial(s, env)?;
                }
                Some(result)
            } else if f == &SUB {
                let mut result = Polynomial::new("", 0.);
                for s in ss {
                    result += &-polynomial(s, env)?;
                }
                Some(result)
            } else if f == &MUL {
                let mut result = Polynomial::new("", 1.);
                for s in ss {
                    result *= &polynomial(s, env)?;
                }
                Some(result)
            } else if f == &POW {
                let fexp: f64 = ss.iter().skip(1).map(eval).sum();
                let exp = if fexp.fract() == 0.0 {
                    fexp as usize
                } else {
                    return None;
                };
                let base = polynomial(&ss[0], env)?;
                let mut result = Polynomial::new("", 1.);
                for _ in 0..exp {
                    result *= &base;
                }
                Some(result)
            } else {
                Some(Polynomial::new("", (f.f)(ss.iter().map(eval).collect())))
            }
        }
        S::Unknown(name) => Some(Polynomial {
            unknown: *name,
            coefs: vec![0., 1.],
        }),
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
