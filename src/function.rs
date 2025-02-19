use std::ops::Neg;

use num::pow::Pow;

use crate::number::Number;

#[derive(Clone, Copy)]
pub struct Function<'f> {
    pub name: &'f str,
    pub arity: u8,
    pub f: F,
    pub priority: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum F {
    Binary(fn(Number, Number) -> Number),
    Nary(fn(Number) -> Number),
}

impl<'f> Function<'f> {
    const fn unary(name: &'f str, priority: u16, f: fn(Number) -> Number) -> Self {
        Self {
            name,
            arity: 1,
            f: F::Nary(f),
            priority,
        }
    }
    const fn binary(name: &'f str, priority: u16, f: fn(Number, Number) -> Number) -> Self {
        Self {
            name,
            arity: 2,
            f: F::Binary(f),
            priority,
        }
    }
}

impl<'f> PartialEq for Function<'f> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'f> Eq for Function<'f> {}

impl<'f> std::fmt::Debug for Function<'f> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}/{}({:?})", self.name, self.arity, self.priority)
    }
}

pub const SQRT: Function = Function::unary("sqrt", 4, |x| Number::Flt(x.f64_or_nan().sqrt()));
pub const CBRT: Function = Function::unary("cbrt", 4, |x| Number::Flt(x.f64_or_nan().cbrt()));
pub const COS: Function = Function::unary("cos", 4, |x| Number::Flt(x.f64_or_nan().cos()));
pub const SIN: Function = Function::unary("sin", 4, |x| Number::Flt(x.f64_or_nan().sin()));
pub const TAN: Function = Function::unary("tan", 4, |x| Number::Flt(x.f64_or_nan().tan()));
pub const COSH: Function = Function::unary("cosh", 4, |x| Number::Flt(x.f64_or_nan().cosh()));
pub const SINH: Function = Function::unary("sinh", 4, |x| Number::Flt(x.f64_or_nan().sinh()));
pub const TANH: Function = Function::unary("tanh", 4, |x| Number::Flt(x.f64_or_nan().tanh()));
pub const ACOS: Function = Function::unary("acos", 4, |x| Number::Flt(x.f64_or_nan().acos()));
pub const ASIN: Function = Function::unary("asin", 4, |x| Number::Flt(x.f64_or_nan().asin()));
pub const ATAN: Function = Function::unary("atan", 4, |x| Number::Flt(x.f64_or_nan().atan()));
pub const ACOSH: Function = Function::unary("acosh", 4, |x| Number::Flt(x.f64_or_nan().acosh()));
pub const ASINH: Function = Function::unary("asinh", 4, |x| Number::Flt(x.f64_or_nan().asinh()));
pub const ATANH: Function = Function::unary("atanh", 4, |x| Number::Flt(x.f64_or_nan().atanh()));
pub const EXP: Function = Function::unary("exp", 4, |x| Number::Flt(x.f64_or_nan().exp()));
pub const LN: Function = Function::unary("ln", 4, |x| Number::Flt(x.f64_or_nan().ln()));
pub const LOG: Function = Function::unary("log", 4, |x| Number::Flt(x.f64_or_nan().log10()));
pub const NEG: Function = Function::unary("-", 6, |x| x.neg());
pub const ID: Function = Function::unary("+", 6, |x| x);
pub const ADD: Function = Function::binary("+", 4, |x, y| y + x);
pub const SUB: Function = Function::binary("-", 4, |x, y| y - x);
pub const MUL: Function = Function::binary("*", 6, |x, y| y * x);
pub const DIV: Function = Function::binary("/", 6, |x, y| y / x);
// https://en.wikipedia.org/wiki/Modulo#Variants_of_the_definition
// Truncated
pub const REM: Function = Function::binary("%", 6, |x, y| y % x);
pub const POW: Function = Function::binary("^", 7, |x, y| y.pow(x));
