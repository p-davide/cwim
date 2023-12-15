use crate::function::*;
use crate::interpreter::Expr;

#[derive(Debug)]
pub enum Variable {
    Function(Function),
    Value(f64),
}

pub struct Env {
    inner: std::collections::HashMap<String, Variable>,
}

impl Env {
    pub fn std() -> Self {
        Self {
            inner: std::collections::HashMap::from([
                ("=".to_owned(), Variable::Function(ASSIGN)),
                ("+".to_owned(), Variable::Function(ADD)),
                ("-".to_owned(), Variable::Function(SUB)),
                ("*".to_owned(), Variable::Function(MUL)),
                ("/".to_owned(), Variable::Function(DIV)),
                ("^".to_owned(), Variable::Function(POW)),
                ("%".to_owned(), Variable::Function(REM)),
                ("sqrt".to_owned(), Variable::Function(SQRT)),
                ("cbrt".to_owned(), Variable::Function(CBRT)),
                ("pi".to_owned(), Variable::Value(std::f64::consts::PI)),
                ("cos".to_owned(), Variable::Function(COS)),
                ("sin".to_owned(), Variable::Function(SIN)),
                ("tan".to_owned(), Variable::Function(TAN)),
                ("exp".to_owned(), Variable::Function(EXP)),
                ("ln".to_owned(), Variable::Function(LN)),
                ("log2".to_owned(), Variable::Function(LOG2)),
                ("acos".to_owned(), Variable::Function(ACOS)),
                ("asin".to_owned(), Variable::Function(ASIN)),
                ("atan".to_owned(), Variable::Function(ATAN)),
                ("arccos".to_owned(), Variable::Function(ACOS)),
                ("arcsin".to_owned(), Variable::Function(ASIN)),
                ("arctan".to_owned(), Variable::Function(ATAN)),
                ("cosh".to_owned(), Variable::Function(COSH)),
                ("sinh".to_owned(), Variable::Function(SINH)),
                ("tanh".to_owned(), Variable::Function(TANH)),
                ("acosh".to_owned(), Variable::Function(ACOSH)),
                ("asinh".to_owned(), Variable::Function(ASINH)),
                ("atanh".to_owned(), Variable::Function(ATANH)),
                ("arccosh".to_owned(), Variable::Function(ACOSH)),
                ("arcsinh".to_owned(), Variable::Function(ASINH)),
                ("arctanh".to_owned(), Variable::Function(ATANH)),
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

    pub fn assign(&mut self, lhs: String, rhs: Variable) -> Option<Variable> {
        self.inner.insert(lhs, rhs)
    }
}
