use crate::function::*;
use crate::interpreter::Expr;

#[derive(Debug)]
pub enum Variable {
    Function(Functions),
    Value(f64),
}

#[derive(Debug)]
pub struct Functions {
    pub unary: Option<Function>,
    pub binary: Option<Function>,
}

pub enum Arity {
    Nullary,
    Unary,
    Binary,
}

impl Functions {
    fn unary(unary: Function) -> Self {
        Self {
            unary: Some(unary),
            binary: None,
        }
    }
    fn binary(binary: Function) -> Self {
        Self {
            binary: Some(binary),
            unary: None,
        }
    }
}
pub struct Env {
    inner: std::collections::HashMap<String, Variable>,
}

fn binary(symbol: &'static str, f: Function) -> (String, Variable) {
    (symbol.to_owned(), Variable::Function(Functions::binary(f)))
}

fn unary(symbol: &'static str, f: Function) -> (String, Variable) {
    (symbol.to_owned(), Variable::Function(Functions::unary(f)))
}

fn value(symbol: &'static str, n: f64) -> (String, Variable) {
    (symbol.to_owned(), Variable::Value(n))
}

impl Env {
    pub fn std() -> Self {
        Self {
            inner: std::collections::HashMap::from([
                binary("=", ASSIGN),
                binary("+", ADD),
                (
                    "-".to_owned(),
                    Variable::Function(Functions {
                        unary: Some(NEG),
                        binary: Some(SUB),
                    }),
                ),
                binary("*", MUL),
                binary("/", DIV),
                binary("^", POW),
                binary("%", REM),
                unary("sqrt", SQRT),
                unary("cbrt", CBRT),
                value("pi", std::f64::consts::PI),
                unary("cos", COS),
                unary("sin", SIN),
                unary("tan", TAN),
                unary("exp", EXP),
                unary("ln", LN),
                unary("log2", LOG2),
                unary("acos", ACOS),
                unary("asin", ASIN),
                unary("atan", ATAN),
                unary("arccos", ACOS),
                unary("arcsin", ASIN),
                unary("arctan", ATAN),
                unary("cosh", COSH),
                unary("sinh", SINH),
                unary("tanh", TANH),
                unary("acosh", ACOSH),
                unary("asinh", ASINH),
                unary("atanh", ATANH),
                unary("arccosh", ACOSH),
                unary("arcsinh", ASINH),
                unary("arctanh", ATANH),
            ]),
        }
    }

    pub fn expr(&self, l: &str, arity: Arity) -> Expr {
        let var = self.inner.get(l);
        match var {
            Some(Variable::Function(f)) => Expr::Function(match arity {
                Arity::Unary => f.unary.expect(l),
                Arity::Binary => f.binary.expect(l),
                Arity::Nullary => unreachable!(),
            }),
            Some(Variable::Value(n)) => Expr::Literal(*n),
            None => Expr::Error(format!("Can't find '{}'", l)),
        }
    }

    pub fn assign(&mut self, lhs: String, rhs: Variable) -> Option<Variable> {
        self.inner.insert(lhs, rhs)
    }
}
