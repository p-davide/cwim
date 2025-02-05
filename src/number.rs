use core::f64;
use std::{fmt::Display, ops::*};

use num::{
    pow::Pow,
    rational::Ratio,
    traits::{ParseFloatError, ToPrimitive},
    BigInt, Num, One, Signed, Zero,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(BigInt),
    Rat(Ratio<BigInt>),
    Flt(f64),
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Number::Flt(x), Number::Flt(y)) => x.partial_cmp(y),
            (Number::Flt(x), Number::Int(y)) => {
                x.partial_cmp(&y.to_f64().unwrap_or(if y.is_negative() {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                }))
            }
            (Number::Flt(x), Number::Rat(y)) => {
                x.partial_cmp(&y.to_f64().unwrap_or(if y.is_negative() {
                    f64::NEG_INFINITY
                } else {
                    f64::INFINITY
                }))
            }
            (Number::Int(_), Number::Flt(_)) => other.partial_cmp(self).map(|c| c.reverse()),
            (Number::Int(x), Number::Int(y)) => x.partial_cmp(y),
            (Number::Int(x), Number::Rat(y)) => Ratio::from_integer(x.clone()).partial_cmp(y),
            (Number::Rat(_), Number::Flt(_)) => other.partial_cmp(self).map(|c| c.reverse()),
            (Number::Rat(_), Number::Int(_)) => other.partial_cmp(self).map(|c| c.reverse()),
            (Number::Rat(x), Number::Rat(y)) => x.partial_cmp(y),
        }
    }
}
impl<T> From<T> for Number
where
    BigInt: From<T>,
{
    fn from(value: T) -> Self {
        Number::Int(BigInt::from(value))
    }
}

impl Num for Number {
    type FromStrRadixErr = ParseFloatError;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, ParseFloatError> {
        if let Ok(n) = <BigInt as Num>::from_str_radix(str, radix) {
            return Ok(Number::Int(n));
        }
        if let Ok(n) = <Ratio<BigInt> as Num>::from_str_radix(str, radix) {
            return Ok(Number::Rat(n));
        }
        let parts: Vec<_> = str.split('.').collect();
        match parts[..] {
            [before, ""] => {
                if let Ok(whole) = <f64 as Num>::from_str_radix(before, radix) {
                    return Ok(Number::Flt(whole));
                }
            }
            ["", after] => {
                if let Ok(decimal) = <BigInt as Num>::from_str_radix(after, radix) {
                    return Ok(Number::Rat(Ratio::new(
                        decimal,
                        BigInt::from(radix.pow(after.len().try_into().unwrap())),
                    )));
                }
            }
            [before, after] => {
                if let (Ok(whole), Ok(decimal)) = (
                    <BigInt as Num>::from_str_radix(before, radix),
                    <BigInt as Num>::from_str_radix(after, radix),
                ) {
                    return Ok(Number::Rat(
                        Ratio::new(
                            decimal,
                            BigInt::from(radix.pow(after.len().try_into().unwrap())),
                        ) + whole,
                    ));
                }
            }
            _ => {}
        }
        match <f64 as Num>::from_str_radix(str, radix) {
            Ok(n) => Ok(Number::Flt(n)),
            Err(msg) => Err(msg),
        }
    }
}

impl Number {
    pub fn try_into_int(&self) -> Self {
        match self {
            Number::Rat(r) if *r.denom() == BigInt::one() => Number::Int(r.numer().clone()),
            _ => self.clone(),
        }
    }
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            Number::Int(big_int) => big_int.to_f64(),
            Number::Rat(ratio) => ratio.to_f64(),
            Number::Flt(x) => Some(*x),
        }
    }
    pub fn f64_or_nan(&self) -> f64 {
        self.to_f64().unwrap_or(std::f64::NAN)
    }
    pub fn is_nan(&self) -> bool {
        match self {
            Self::Flt(n) if n.is_nan() => true,
            _ => false,
        }
    }
}

impl Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x + y.f64_or_nan()),
            (x, Number::Flt(y)) => Number::Flt(x.f64_or_nan() + y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x + y),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x + y),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(y + x),
            (Number::Int(x), Number::Int(y)) => Number::Int(x + y),
        }
        .try_into_int()
    }
}
impl Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x - y.f64_or_nan()),
            (x, Number::Flt(y)) => Number::Flt(x.f64_or_nan() - y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x - y),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x - y),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(-y + x),
            (Number::Int(x), Number::Int(y)) => Number::Int(x - y),
        }
        .try_into_int()
    }
}
impl Mul for Number {
    type Output = Number;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x * y.f64_or_nan()),
            (x, Number::Flt(y)) => Number::Flt(x.f64_or_nan() * y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x * y),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x * y),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(y * x),
            (Number::Int(x), Number::Int(y)) => Number::Int(x * y),
        }
        .try_into_int()
    }
}
impl Div for Number {
    type Output = Number;
    fn div(self, rhs: Self) -> Self::Output {
        if (&rhs).is_zero() {
            eprintln!("Attempted to divide {} by zero!", self);
            return Self::Flt(f64::NAN);
        }
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x / y.f64_or_nan()),
            (x, Number::Flt(y)) => Number::Flt(x.f64_or_nan() / y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x / y),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x / y),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(y.recip() * x),
            (Number::Int(x), Number::Int(y)) => Number::Rat(Ratio::new(x.clone(), y.clone())),
        }
        .try_into_int()
    }
}

impl Pow<Number> for Number {
    type Output = Number;
    fn pow(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), Number::Flt(y)) => Number::Flt(x.powf(y)),
            (Number::Flt(x), Number::Int(y)) => {
                Number::Flt(x.powi(y.to_i32().unwrap_or(std::i32::MIN)))
            }
            (Number::Flt(x), Number::Rat(y)) => {
                Number::Flt(x.powf(y.to_f64().unwrap_or(std::f64::NAN)))
            }
            (x, Number::Flt(y)) => Number::Flt(x.f64_or_nan().powf(y)),
            (Number::Rat(x), Number::Rat(y)) => Number::Flt(
                x.to_f64()
                    .unwrap_or(std::f64::NAN)
                    .powf(y.to_f64().unwrap_or(std::f64::NAN)),
            ),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(Ratio::new(
                x.numer().pow(y.to_u32().unwrap_or(0)),
                x.denom().pow(y.to_u32().unwrap_or(0)),
            ))
            .try_into_int(),
            (Number::Int(x), Number::Rat(y)) => Number::Flt(
                x.to_f64()
                    .unwrap_or(std::f64::NAN)
                    .powf(y.to_f64().unwrap_or(std::f64::NAN)),
            ),
            (Number::Int(x), Number::Int(y)) => {
                if y >= BigInt::ZERO {
                    let yy = y.to_u32().unwrap_or(0);
                    Number::Int(x.pow(yy))
                } else {
                    Number::Int(BigInt::one().div(&(x.pow(y.to_u32().unwrap_or(0)))))
                }
            }
        }
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x % y.to_f64().unwrap_or(std::f64::NAN)),
            (x, Number::Flt(y)) => Number::Flt(x.to_f64().unwrap_or(std::f64::NAN) % y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x % y).try_into_int(),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x % y).try_into_int(),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(y % x).try_into_int(),
            (Number::Int(x), Number::Int(y)) => Number::Int(x % y).try_into_int(),
        }
    }
}

impl Neg for Number {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Number::zero() - self
    }
}

impl One for Number {
    fn one() -> Self {
        Number::Int(BigInt::one())
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Number::Int(BigInt::ZERO)
    }

    fn is_zero(&self) -> bool {
        match self {
            Number::Int(n) => n.is_zero(),
            Number::Rat(n) => n.is_zero(),
            Number::Flt(n) => n.is_zero(),
        }
    }
}

impl Signed for Number {
    fn abs(&self) -> Self {
        match self {
            Number::Int(n) => Number::Int(n.abs()),
            Number::Rat(n) => Number::Rat(n.abs()),
            Number::Flt(n) => Number::Flt(n.abs()),
        }
    }

    fn abs_sub(&self, other: &Self) -> Self {
        let sub = self.clone() - other.clone();
        if sub.is_positive() {
            sub
        } else {
            Self::zero()
        }
    }

    fn signum(&self) -> Self {
        if self.is_positive() {
            return Self::one();
        }
        if self.is_negative() {
            return -Self::one();
        }
        if self.is_zero() {
            return Self::zero();
        }
        Self::Flt(f64::NAN)
    }

    fn is_positive(&self) -> bool {
        match self {
            Number::Int(n) => n.is_positive(),
            Number::Rat(n) => n.is_positive(),
            Number::Flt(n) => n.is_positive(),
        }
    }

    fn is_negative(&self) -> bool {
        match self {
            Number::Int(n) => n.is_negative(),
            Number::Rat(n) => n.is_negative(),
            Number::Flt(n) => n.is_negative(),
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(n) => n.fmt(f),
            Self::Rat(n) => n.fmt(f),
            Self::Flt(n) => n.fmt(f),
        }
    }
}
