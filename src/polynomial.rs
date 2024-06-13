use std::{
    fmt::Debug,
    iter::{repeat, zip},
    ops::*,
};

use num::{One, Zero};

use crate::{
    env,
    function::{ADD, MUL, POW, SUB},
    number::Number,
    parser::Parsed,
    s::{eval, S},
};

#[derive(Debug, Clone)]
pub struct Polynomial<'a> {
    pub unknown: &'a str,
    coefs: Vec<Number>,
}

// The 1st element gives the 0th grade coefficient
impl<'a> Polynomial<'a> {
    pub fn new(unknown: &'a str, n: Number) -> Self {
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
    pub fn roots(&self) -> Vec<Number> {
        let inner = match &self.coefs[..] {
            [] => vec![],
            [_] => vec![],
            [b, a] => vec![-(*b / *a)],
            [c, b, a] => {
                let delta = ((*b * *b) - ((*a * *c) * Number::Int(4))).wrapped(|x| x.sqrt());
                // TODO: complex solutions
                match delta {
                    delta @ Number::Flt(d) if !d.is_nan() => vec![
                        (-(*b - delta)) / (Number::Int(2) * *a),
                        (-(*b + delta)) / (Number::Int(2) * *a),
                    ],
                    _ => vec![],
                }
            }
            // TODO: higher order polynomials
            _ => vec![],
        };
        // TODO: multiple solutions
        inner
    }
}

impl<'a> Add<Self> for &Polynomial<'a> {
    type Output = Polynomial<'a>;
    fn add(self, other: Self) -> Self::Output {
        let (longest, shortest) = if self.coefs.len() > other.coefs.len() {
            (&(self.coefs), &(other.coefs))
        } else {
            (&(other.coefs), &(self.coefs))
        };
        let coefs = zip(
            longest.iter(),
            shortest.iter().chain(repeat(&Number::zero())),
        )
        .map(|(x, y)| *x + *y)
        .collect();
        let mut result = Polynomial { coefs, ..*self };
        result.set_unknown(other.unknown);
        result
    }
}

impl<'a> AddAssign<&Self> for Polynomial<'a> {
    fn add_assign(&mut self, rhs: &Self) {
        let result = &*self + rhs;
        self.coefs = result.coefs;
        self.unknown = result.unknown;
    }
}

impl<'a> Sub<Self> for &Polynomial<'a> {
    type Output = Polynomial<'a>;
    fn sub(self, other: Self) -> Self::Output {
        let (longest, shortest) = if self.coefs.len() > other.coefs.len() {
            (&(self.coefs), &(other.coefs))
        } else {
            (&(other.coefs), &(self.coefs))
        };
        let coefs = zip(
            longest.iter(),
            shortest.iter().chain(repeat(&Number::zero())),
        )
        .map(|(x, y)| *x - *y)
        .collect();
        let mut result = Polynomial { coefs, ..*self };
        result.set_unknown(other.unknown);
        result
    }
}

impl<'a> SubAssign<&Self> for Polynomial<'a> {
    fn sub_assign(&mut self, rhs: &Self) {
        let result = &*self - rhs;
        self.coefs = result.coefs;
        self.unknown = result.unknown;
    }
}

impl<'a> Mul<Self> for &Polynomial<'a> {
    type Output = Polynomial<'a>;
    fn mul(self, other: Self) -> Self::Output {
        let mut result = vec![Number::zero(); self.coefs.len() + other.coefs.len() - 1];
        for (d1, c1) in self.coefs.iter().enumerate() {
            for (d2, c2) in other.coefs.iter().enumerate() {
                result[d1 + d2] = *c1 * *c2 + result[d1 + d2];
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

impl<'a> MulAssign<&Self> for Polynomial<'a> {
    fn mul_assign(&mut self, rhs: &Self) {
        let result = &*self * &rhs;
        self.coefs = result.coefs;
        self.unknown = result.unknown;
    }
}

impl<'a> Add<Number> for Polynomial<'a> {
    type Output = Self;
    fn add(self, other: Number) -> Self::Output {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl<'a> AddAssign<Number> for Polynomial<'a> {
    fn add_assign(&mut self, other: Number) {
        self.coefs[0] = self.coefs[0] + other;
    }
}

impl<'a> SubAssign<Number> for Polynomial<'a> {
    fn sub_assign(&mut self, other: Number) {
        self.coefs[0] = self.coefs[0] - other;
    }
}

impl<'a> Neg for Polynomial<'a> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::Output {
            coefs: self.coefs.iter().map(|x| x.neg()).collect(),
            ..self
        }
    }
}

pub fn polynomial<'a>(s: &S<'a>, env: &env::Env) -> Parsed<Polynomial<'a>> {
    match s {
        S::Var(n) => Ok(Polynomial::new("", n.clone())),
        S::Fun(fun, ss) => {
            if fun == &ADD {
                let mut result = Polynomial::new("", Number::zero());
                for s in ss {
                    result += &polynomial(s, env)?;
                }
                Ok(result)
            } else if fun == &SUB {
                let mut result = Polynomial::new("", Number::zero());
                for s in ss {
                    result -= &polynomial(s, env)?;
                }
                Ok(result)
            } else if fun == &MUL {
                let mut result = Polynomial::new("", Number::one());
                for s in ss {
                    result *= &polynomial(s, env)?;
                }
                Ok(result)
            } else if fun == &POW {
                // TODO: Matrix exponents
                let fexp = 0f64;
                for s in ss.iter().skip(1) {
                    match &eval(s)? {
                        bad => return Err(format!("Exponent should have size 1, found {:?}", bad)),
                    }
                }
                let exp = if fexp.fract() == 0.0 {
                    fexp as usize
                } else {
                    return Err("Fractional exponent are not supported".to_owned());
                };
                let base = polynomial(&ss[0], env)?;
                let mut result = Polynomial::new("", Number::one());
                for _ in 0..exp {
                    result *= &base;
                }
                Ok(result)
            } else {
                Ok(Polynomial::new("", eval(s)?))
            }
        }
        S::Unknown(name) => Ok(Polynomial {
            unknown: *name,
            coefs: vec![Number::zero(), Number::one()],
        }),
    }
}
