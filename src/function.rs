use num_traits::{real::Real, Num};

#[derive(Clone, Copy)]
pub struct Function<N> {
    pub name: &'static str,
    pub arity: u8,
    pub f: fn(Vec<N>) -> N,
    pub priority: u16,
}

impl<N> PartialEq for Function<N> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<N> Eq for Function<N> {}

impl<N> std::fmt::Debug for Function<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}/{}({:?})", self.name, self.arity, self.priority)
    }
}

pub fn SQRT<N: Num + Real>() -> Function<N> {
    Function {
        name: "sqrt",
        arity: 1,
        f: |x| x[0].sqrt(),
        priority: (2),
    }
}

pub fn CBRT<N: Num + Real>() -> Function<N> {
    Function {
        name: "cbrt",
        arity: 1,
        f: |x| x[0].cbrt(),
        priority: (2),
    }
}

pub fn COS<N: Num + Real>() -> Function<N> {
    Function {
        name: "cos",
        arity: 1,
        f: |x| x[0].cos(),
        priority: (2),
    }
}

pub fn SIN<N: Num + Real>() -> Function<N> {
    Function {
        name: "sin",
        arity: 1,
        f: |x| x[0].sin(),
        priority: (2),
    }
}

pub fn TAN<N: Num + Real>() -> Function<N> {
    Function {
        name: "tan",
        arity: 1,
        f: |x| x[0].tan(),
        priority: (2),
    }
}

pub fn EXP<N: Num + Real>() -> Function<N> {
    Function {
        name: "exp",
        arity: 1,
        f: |x| x[0].exp(),
        priority: (2),
    }
}

pub fn LN<N: Num + Real>() -> Function<N> {
    Function {
        name: "ln",
        arity: 1,
        f: |x| x[0].ln(),
        priority: (2),
    }
}

pub fn LOG<N: Num + Real>() -> Function<N> {
    Function {
        name: "log",
        arity: 1,
        f: |x| x[0].log10(),
        priority: (2),
    }
}

pub fn ACOS<N: Num + Real>() -> Function<N> {
    Function {
        name: "acos",
        arity: 1,
        f: |x| x[0].acos(),
        priority: (2),
    }
}

pub fn ASIN<N: Num + Real>() -> Function<N> {
    Function {
        name: "asin",
        arity: 1,
        f: |x| x[0].asin(),
        priority: (2),
    }
}

pub fn ATAN<N: Num + Real>() -> Function<N> {
    Function {
        name: "atan",
        arity: 1,
        f: |x| x[0].atan(),
        priority: (2),
    }
}

pub fn COSH<N: Num + Real>() -> Function<N> {
    Function {
        name: "cosh",
        arity: 1,
        f: |x| x[0].cosh(),
        priority: (2),
    }
}

pub fn SINH<N: Num + Real>() -> Function<N> {
    Function {
        name: "sinh",
        arity: 1,
        f: |x| x[0].sinh(),
        priority: (2),
    }
}

pub fn TANH<N: Num + Real>() -> Function<N> {
    Function {
        name: "tanh",
        arity: 1,
        f: |x| x[0].tanh(),
        priority: (2),
    }
}

pub fn ACOSH<N: Num + Real>() -> Function<N> {
    Function {
        name: "acosh",
        arity: 1,
        f: |x| x[0].acosh(),
        priority: (2),
    }
}

pub fn ASINH<N: Num + Real>() -> Function<N> {
    Function {
        name: "asinh",
        arity: 1,
        f: |x| x[0].asinh(),
        priority: (2),
    }
}

pub fn ATANH<N: Num + Real>() -> Function<N> {
    Function {
        name: "atanh",
        arity: 1,
        f: |x| x[0].atanh(),
        priority: (2),
    }
}

pub fn NEG<N: Num + Real>() -> Function<N> {
    Function {
        name: "-",
        arity: 1,
        f: |x| -x[0],
        priority: 6,
    }
}

pub fn ID<N: Num + Real>() -> Function<N> {
    Function {
        name: "+",
        arity: 1,
        f: |x| x[0],
        priority: 6,
    }
}

pub fn ADD<N: Num + Real>() -> Function<N> {
    Function {
        name: "+",
        arity: 2,
        f: |xs| xs[1] + xs[0],
        priority: (4),
    }
}

pub fn SUB<N: Num + Real>() -> Function<N> {
    Function {
        name: "-",
        arity: 2,
        f: |xs| xs[1] - xs[0],
        priority: (4),
    }
}

pub fn MUL<N: Num + Real>() -> Function<N> {
    Function {
        name: "*",
        arity: 2,
        f: |xs| xs[1] * xs[0],
        priority: 6,
    }
}

pub fn DIV<N: Num + Real>() -> Function<N> {
    Function {
        name: "/",
        arity: 2,
        f: |xs| xs[1] / xs[0],
        priority: 6,
    }
}

// https://en.wikipedia.org/wiki/Modulo#Variants_of_the_definition
// Truncated
pub fn REM<N: Num + Real>() -> Function<N> {
    Function {
        name: "%",
        arity: 2,
        f: |xs| xs[1] % xs[0],
        priority: 6,
    }
}

pub fn POW<N: Num + Real>() -> Function<N> {
    Function {
        name: "^",
        arity: 2,
        f: |xs| xs[1].powf(xs[0]),
        priority: 7,
    }
}
