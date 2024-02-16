use num_traits::real::Real;
use num_traits::Num;

use crate::function::*;
use crate::interpreter::Expr;
use crate::parser::Parsed;

#[derive(Debug)]
pub enum Variable<N> {
    Function(Functions<N>),
    Value(N),
}

#[derive(Debug)]
pub struct Functions<N> {
    pub unary: Option<Function<N>>,
    pub binary: Option<Function<N>>,
}

impl<N> Functions<N> {
    fn unary(unary: Function<N>) -> Self {
        Self {
            unary: Some(unary),
            binary: None,
        }
    }
    fn binary(binary: Function<N>) -> Self {
        Self {
            binary: Some(binary),
            unary: None,
        }
    }
}
pub struct Env<N> {
    inner: std::collections::HashMap<String, Variable<N>>,
}

fn binary<N>(symbol: &'static str, f: Function<N>) -> (String, Variable<N>) {
    (symbol.to_owned(), Variable::Function(Functions::binary(f)))
}

fn unary<N>(symbol: &'static str, f: Function<N>) -> (String, Variable<N>) {
    (symbol.to_owned(), Variable::Function(Functions::unary(f)))
}

fn value<N>(symbol: &'static str, n: N) -> (String, Variable<N>) {
    (symbol.to_owned(), Variable::Value(n))
}

impl<N: Num + Real> Env<N> {
    pub fn prelude() -> Self {
        Self {
            inner: std::collections::HashMap::from([
                (
                    "+".to_owned(),
                    Variable::Function(Functions {
                        unary: Some(ID()),
                        binary: Some(ADD()),
                    }),
                ),
                (
                    "-".to_owned(),
                    Variable::Function(Functions {
                        unary: Some(NEG()),
                        binary: Some(SUB()),
                    }),
                ),
                binary("*", MUL()),
                binary("/", DIV()),
                binary("^", POW()),
                binary("%", REM()),
                unary("sqrt", SQRT()),
                unary("cbrt", CBRT()),
                //value("pi", std::f64::consts::PI),
                unary("cos", COS()),
                unary("sin", SIN()),
                unary("tan", TAN()),
                unary("exp", EXP()),
                unary("ln", LN()),
                unary("log", LOG()),
                unary("acos", ACOS()),
                unary("asin", ASIN()),
                unary("atan", ATAN()),
                unary("arccos", ACOS()),
                unary("arcsin", ASIN()),
                unary("arctan", ATAN()),
                unary("cosh", COSH()),
                unary("sinh", SINH()),
                unary("tanh", TANH()),
                unary("acosh", ACOSH()),
                unary("asinh", ASINH()),
                unary("atanh", ATANH()),
                unary("arccosh", ACOSH()),
                unary("arcsinh", ASINH()),
                unary("arctanh", ATANH()),
            ]),
        }
    }

    pub fn find_value(&self, l: &str) -> Parsed<Expr<N>> {
        let var = self.inner.get(l);
        match var {
            Some(Variable::Value(n)) => Ok(Expr::Literal(*n)),
            Some(Variable::Function(_)) => Err(format!(
                "Expected value '{}', found function with that name.",
                l
            )),
            None => Err(format!("Can't find '{}'", l)),
        }
    }

    pub fn find_unary_or_literal(&self, l: &str) -> Parsed<Expr<N>> {
        let var = self.inner.get(l);
        match var {
            Some(Variable::Function(Functions {
                binary: _,
                unary: Some(unary),
            })) => Ok(Expr::Function(*unary)),
            Some(Variable::Value(n)) => Ok(Expr::Literal(*n)),
            _ => Err(format!("Can't find '{}'", l)),
        }
    }

    pub fn find_binary_or_literal(&self, l: &str) -> Parsed<Expr<N>> {
        let var = self.inner.get(l);
        match var {
            Some(Variable::Function(Functions {
                unary: _,
                binary: Some(binary),
            })) => Ok(Expr::Function(*binary)),
            Some(Variable::Value(n)) => Ok(Expr::Literal(*n)),
            _ => Err(format!("Can't find '{}'", l)),
        }
    }

    pub fn assign(&mut self, lhs: String, rhs: N) -> Option<Variable<N>> {
        self.inner.insert(lhs, Variable::Value(rhs))
    }
}
