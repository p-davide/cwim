use crate::prioritize::Priority;

#[derive(Clone, Copy)]
pub struct Function {
    pub name: &'static str,
    pub arity: u8,
    pub f: fn(Vec<f64>) -> f64,
    pub precedence: Priority,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Function {}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}/{}({:?})", self.name, self.arity, self.precedence)
    }
}

impl Function {
    pub fn was_spaced(&self) -> bool {
        self.precedence.spaces > 0
    }
    pub fn paren(&mut self) -> Self {
        self.precedence.parens += 1;
        *self
    }
    pub fn space(&mut self) -> Self {
        self.precedence.spaces += 1;
        *self
    }
}

pub const SQRT: Function = Function {
    name: "sqrt",
    arity: 1,
    f: |x| x[0].sqrt(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const CBRT: Function = Function {
    name: "cbrt",
    arity: 1,
    f: |x| x[0].cbrt(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const COS: Function = Function {
    name: "cos",
    arity: 1,
    f: |x| x[0].cos(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const SIN: Function = Function {
    name: "sin",
    arity: 1,
    f: |x| x[0].sin(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const TAN: Function = Function {
    name: "tan",
    arity: 1,
    f: |x| x[0].tan(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const EXP: Function = Function {
    name: "exp",
    arity: 1,
    f: |x| x[0].exp(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const LN: Function = Function {
    name: "ln",
    arity: 1,
    f: |x| x[0].ln(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const LOG: Function = Function {
    name: "log",
    arity: 1,
    f: |x| x[0].log10(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const ACOS: Function = Function {
    name: "acos",
    arity: 1,
    f: |x| x[0].acos(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const ASIN: Function = Function {
    name: "asin",
    arity: 1,
    f: |x| x[0].asin(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const ATAN: Function = Function {
    name: "atan",
    arity: 1,
    f: |x| x[0].atan(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const COSH: Function = Function {
    name: "cosh",
    arity: 1,
    f: |x| x[0].cosh(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const SINH: Function = Function {
    name: "sinh",
    arity: 1,
    f: |x| x[0].sinh(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const TANH: Function = Function {
    name: "tanh",
    arity: 1,
    f: |x| x[0].tanh(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const ACOSH: Function = Function {
    name: "acosh",
    arity: 1,
    f: |x| x[0].acosh(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const ASINH: Function = Function {
    name: "asinh",
    arity: 1,
    f: |x| x[0].asinh(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const ATANH: Function = Function {
    name: "atanh",
    arity: 1,
    f: |x| x[0].atanh(),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const NEG: Function = Function {
    name: "-",
    arity: 1,
    f: |x| -x[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: 6,
    },
};

pub const ID: Function = Function {
    name: "+",
    arity: 1,
    f: |x| x[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (2),
    },
};

pub const ADD: Function = Function {
    name: "+",
    arity: 2,
    f: |xs| xs[1] + xs[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (4),
    },
};

pub const SUB: Function = Function {
    name: "-",
    arity: 2,
    f: |xs| xs[1] - xs[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (4),
    },
};

pub const MUL: Function = Function {
    name: "*",
    arity: 2,
    f: |xs| xs[1] * xs[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: 6,
    },
};

pub const DIV: Function = Function {
    name: "/",
    arity: 2,
    f: |xs| xs[1] / xs[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: 6,
    },
};

// https://en.wikipedia.org/wiki/Modulo#Variants_of_the_definition
// Truncated
pub const REM: Function = Function {
    name: "%",
    arity: 2,
    f: |xs| xs[1] % xs[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: 6,
    },
};

pub const POW: Function = Function {
    name: "^",
    arity: 2,
    f: |xs| xs[1].powf(xs[0]),
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: 7,
    },
};

pub const ASSIGN: Function = Function {
    name: "=",
    arity: 2,
    f: |xs| xs[0],
    precedence: Priority {
        spaces: 0,
        parens: 0,
        op_priority: (0),
    },
};
