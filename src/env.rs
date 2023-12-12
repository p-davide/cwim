use crate::function::*;
use crate::interpreter::Expr;

pub enum Variable {
    Function(Function),
    Value(f64),
}

pub struct Env<'e> {
    inner: std::collections::HashMap<&'e str, Variable>,
}

impl<'e> Env<'e> {
    pub fn std() -> Self {
        Self {
            inner: std::collections::HashMap::from([
                ("+", Variable::Function(ADD)),
                ("-", Variable::Function(SUB)),
                ("*", Variable::Function(MUL)),
                ("/", Variable::Function(DIV)),
                ("^", Variable::Function(POW)),
                ("pi", Variable::Value(std::f64::consts::PI)),
                ("cos", Variable::Function(COS)),
                ("sin", Variable::Function(SIN)),
                ("tan", Variable::Function(TAN)),
                ("exp", Variable::Function(EXP)),
                ("ln", Variable::Function(LN)),
                ("log2", Variable::Function(LOG2)),
                ("acos", Variable::Function(ACOS)),
                ("asin", Variable::Function(ASIN)),
                ("atan", Variable::Function(ATAN)),
                ("arccos", Variable::Function(ACOS)),
                ("arcsin", Variable::Function(ASIN)),
                ("arctan", Variable::Function(ATAN)),
                ("cosh", Variable::Function(COSH)),
                ("sinh", Variable::Function(SINH)),
                ("tanh", Variable::Function(TANH)),
                ("acosh", Variable::Function(ACOSH)),
                ("asinh", Variable::Function(ASINH)),
                ("atanh", Variable::Function(ATANH)),
                ("arccosh", Variable::Function(ACOSH)),
                ("arcsinh", Variable::Function(ASINH)),
                ("arctanh", Variable::Function(ATANH)),
            ]),
        }
    }

    pub fn expr(&self, l: &str) -> Expr {
        let var = self.inner.get(l);
        match var {
            Some(Variable::Function(f)) => Expr::Function(*f),
            Some(Variable::Value(n)) => Expr::Literal(*n),
            None => Expr::Error(format!("Can't find '{}'", l)),
        }
    }
}
