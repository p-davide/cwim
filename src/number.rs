use std::fmt::Display;
use std::ops::{Add,Sub,Mul,Div,Rem,Neg};

#[derive(Clone, PartialEq)]
pub struct Number {
    pub inner: Vec<f64>,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner.len() {
            1 => std::fmt::Display::fmt(&self.inner[0], f),
            _ => std::fmt::Debug::fmt(&self.inner, f),
        }
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner.len() {
            1 => std::fmt::Display::fmt(&self.inner[0], f),
            _ => std::fmt::Debug::fmt(&self.inner, f),
        }
    }
}

impl Number {
    pub fn scalar(n: f64) -> Self {
        Self { inner: vec![n] }
    }
    pub fn vectorized(&self, f: fn(f64) -> f64) -> Self {
        Self {
            inner: self.inner.iter().map(|x| f(*x)).collect(),
        }
    }
}

impl Add for &Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        Number {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(x, y)| (x + y))
                .collect(),
        }
    }
}

impl Sub for &Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        Number {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(x, y)| (x - y))
                .collect(),
        }
    }
}

impl Mul for &Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        Number {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(x, y)| (x * y))
                .collect(),
        }
    }
}

impl Div for &Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        Number {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(x, y)| (x / y))
                .collect(),
        }
    }
}

impl Rem for &Number {
    type Output = Number;

    fn rem(self, rhs: Self) -> Self::Output {
        Number {
            inner: self
                .inner
                .iter()
                .zip(rhs.inner.iter())
                .map(|(x, y)| (x % y))
                .collect(),
        }
    }
}

impl Neg for &Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        Number {
            inner: self.inner.iter().map(|x| -x).collect(),
        }
    }
}
