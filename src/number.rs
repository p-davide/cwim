use std::{f64::NAN, fmt::Display, ops::*};

use num::{
    pow::Pow,
    rational::Ratio,
    traits::{ParseFloatError, ToPrimitive},
    Num, One, Zero,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Number {
    Int(i64),
    Rat(Ratio<i64>),
    Flt(f64),
}

impl Num for Number {
    type FromStrRadixErr = ParseFloatError;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if let Ok(n) = <i64 as Num>::from_str_radix(str, radix) {
            return Ok(Number::Int(n));
        }
        if let Ok(n) = <Ratio<i64> as Num>::from_str_radix(str, radix) {
            return Ok(Number::Rat(n));
        }
        let parts: Vec<_> = str.split('.').collect();
        match parts[..] {
            [before, ""] => {
                if let Ok(whole) = <i64 as Num>::from_str_radix(before, radix) {
                    return Ok(Number::Flt(whole as f64));
                }
            }
            ["", after] => {
                if let Ok(decimal) = <i64 as Num>::from_str_radix(after, radix) {
                    return Ok(Number::Rat(Ratio::new(
                        decimal,
                        radix.pow(after.len().try_into().unwrap()) as i64,
                    )));
                }
            }
            [before, after] => {
                if let (Ok(whole), Ok(decimal)) = (
                    <i64 as Num>::from_str_radix(before, radix),
                    <i64 as Num>::from_str_radix(after, radix),
                ) {
                    return Ok(Number::Rat(
                        Ratio::new(decimal, radix.pow(after.len().try_into().unwrap()) as i64)
                            + whole,
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
    pub fn to_f64_unchecked(&self) -> f64 {
        match self {
            Number::Int(x) => match x.to_f64() {
                None => {
                    eprintln!("{} can't be converted to a float", x);
                    0.0
                }
                Some(x) => x,
            },
            Number::Rat(x) => match x.to_f64() {
                None => {
                    eprintln!("{} can't be converted to a float", x);
                    0.0
                }
                Some(x) => x,
            },
            Number::Flt(x) => *x,
        }
    }
    pub fn wrapped(&self, f: fn(f64) -> f64) -> Number {
        Number::Flt(f(self.to_f64_unchecked()))
    }
    pub fn try_into_int(&self) -> Number {
        match self {
            Number::Rat(r) if *r.denom() == 1 => Number::Int(*r.numer()),
            _ => *self,
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x + y.to_f64_unchecked()),
            (x, Number::Flt(y)) => Number::Flt(x.to_f64_unchecked() + y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x + y).try_into_int(),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x + y).try_into_int(),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(y + x).try_into_int(),
            (Number::Int(x), Number::Int(y)) => Number::Int(x + y),
        }
    }
}

impl Sub for Number {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x - y.to_f64_unchecked()),
            (x, Number::Flt(y)) => Number::Flt(x.to_f64_unchecked() - y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x - y).try_into_int(),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x - y).try_into_int(),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(-y + x).try_into_int(),
            (Number::Int(x), Number::Int(y)) => Number::Int(x - y),
        }
    }
}

impl Mul for Number {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x * y.to_f64_unchecked()),
            (x, Number::Flt(y)) => Number::Flt(x.to_f64_unchecked() * y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x * y).try_into_int(),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x * y).try_into_int(),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(y * x).try_into_int(),
            (Number::Int(x), Number::Int(y)) => Number::Int(x * y),
        }
    }
}

impl Div for Number {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x / y.to_f64_unchecked()),
            (x, Number::Flt(y)) => Number::Flt(x.to_f64_unchecked() / y),
            (Number::Rat(x), Number::Rat(y)) => Number::Rat(x / y).try_into_int(),
            (Number::Rat(x), Number::Int(y)) => Number::Rat(x / y).try_into_int(),
            (Number::Int(x), Number::Rat(y)) => Number::Rat(y.recip() * x).try_into_int(),
            (Number::Int(x), Number::Int(y)) => Number::Rat(Ratio::new(x, y)).try_into_int(),
        }
    }
}

impl Pow<Number> for Number {
    type Output = Number;

    fn pow(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x.powf(y.to_f64_unchecked())),
            (x, Number::Flt(y)) => Number::Flt(x.to_f64_unchecked().powf(y)),
            (Number::Rat(x), Number::Rat(y)) => {
                Number::Flt(x.to_f64().unwrap().powf(y.to_f64().unwrap()))
            }
            (Number::Rat(x), Number::Int(y)) => {
                Number::Rat(Ratio::new(x.numer().pow(y as u32), x.denom().pow(y as u32)))
                    .try_into_int()
            }
            (Number::Int(x), Number::Rat(y)) => {
                Number::Flt(x.to_f64().unwrap().powf(y.to_f64().unwrap()))
            }
            (Number::Int(x), Number::Int(y)) => {
                if y < 0 {
                    Number::one() / self.pow(-rhs)
                } else {
                    // TODO: don't panic
                    match y.try_into() {
                        Ok(y) => Number::Int(x.pow(y)),
                        Err(_) => {
                            eprintln!("Overflow trying to raise: {} to the exponent: {}", x, y);
                            Number::Flt(NAN)
                        }
                    }
                }
            }
        }
    }
}

impl Rem for Number {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Number::Flt(x), y) => Number::Flt(x % y.to_f64_unchecked()),
            (x, Number::Flt(y)) => Number::Flt(x.to_f64_unchecked() % y),
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
        Number::Int(1)
    }
}

impl Zero for Number {
    fn zero() -> Self {
        Number::Int(0)
    }

    fn is_zero(&self) -> bool {
        match self {
            Number::Int(n) => *n != 0.into(),
            Number::Rat(n) => *n != 0.into(),
            Number::Flt(n) => *n != 0.into(),
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
