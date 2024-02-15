#[derive(Clone, Copy)]
pub struct Function {
    pub name: &'static str,
    pub arity: u8,
    pub f: fn(Vec<f64>) -> f64,
    pub priority: u16,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Function {}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}/{}({:?})", self.name, self.arity, self.priority)
    }
}

pub const SQRT: Function = Function {
    name: "sqrt",
    arity: 1,
    f: |x| x[0].sqrt(),
    priority: (2),
};

pub const CBRT: Function = Function {
    name: "cbrt",
    arity: 1,
    f: |x| x[0].cbrt(),
    priority: (2),
};

pub const COS: Function = Function {
    name: "cos",
    arity: 1,
    f: |x| x[0].cos(),
    priority: (2),
};

pub const SIN: Function = Function {
    name: "sin",
    arity: 1,
    f: |x| x[0].sin(),
    priority: (2),
};

pub const TAN: Function = Function {
    name: "tan",
    arity: 1,
    f: |x| x[0].tan(),
    priority: (2),
};

pub const EXP: Function = Function {
    name: "exp",
    arity: 1,
    f: |x| x[0].exp(),
    priority: (2),
};

pub const LN: Function = Function {
    name: "ln",
    arity: 1,
    f: |x| x[0].ln(),
    priority: (2),
};

pub const LOG: Function = Function {
    name: "log",
    arity: 1,
    f: |x| x[0].log10(),
    priority: (2),
};

pub const ACOS: Function = Function {
    name: "acos",
    arity: 1,
    f: |x| x[0].acos(),
    priority: (2),
};

pub const ASIN: Function = Function {
    name: "asin",
    arity: 1,
    f: |x| x[0].asin(),
    priority: (2),
};

pub const ATAN: Function = Function {
    name: "atan",
    arity: 1,
    f: |x| x[0].atan(),
    priority: (2),
};

pub const COSH: Function = Function {
    name: "cosh",
    arity: 1,
    f: |x| x[0].cosh(),
    priority: (2),
};

pub const SINH: Function = Function {
    name: "sinh",
    arity: 1,
    f: |x| x[0].sinh(),
    priority: (2),
};

pub const TANH: Function = Function {
    name: "tanh",
    arity: 1,
    f: |x| x[0].tanh(),
    priority: (2),
};

pub const ACOSH: Function = Function {
    name: "acosh",
    arity: 1,
    f: |x| x[0].acosh(),
    priority: (2),
};

pub const ASINH: Function = Function {
    name: "asinh",
    arity: 1,
    f: |x| x[0].asinh(),
    priority: (2),
};

pub const ATANH: Function = Function {
    name: "atanh",
    arity: 1,
    f: |x| x[0].atanh(),
    priority: (2),
};

pub const NEG: Function = Function {
    name: "-",
    arity: 1,
    f: |x| -x[0],
    priority: 6,
};

pub const ID: Function = Function {
    name: "+",
    arity: 1,
    f: |x| x[0],
    priority: 6,
};

pub const ADD: Function = Function {
    name: "+",
    arity: 2,
    f: |xs| xs[1] + xs[0],
    priority: (4),
};

pub const SUB: Function = Function {
    name: "-",
    arity: 2,
    f: |xs| xs[1] - xs[0],
    priority: (4),
};

pub const MUL: Function = Function {
    name: "*",
    arity: 2,
    f: |xs| xs[1] * xs[0],
    priority: 6,
};

pub const DIV: Function = Function {
    name: "/",
    arity: 2,
    f: |xs| xs[1] / xs[0],
    priority: 6,
};

// https://en.wikipedia.org/wiki/Modulo#Variants_of_the_definition
// Truncated
pub const REM: Function = Function {
    name: "%",
    arity: 2,
    f: |xs| xs[1] % xs[0],
    priority: 6,
};

pub const POW: Function = Function {
    name: "^",
    arity: 2,
    f: |xs| xs[1].powf(xs[0]),
    priority: 7,
};

