use crate::prioritize::PRIORITY_SPACE;

#[derive(Clone, Copy)]
pub struct Function {
    pub name: &'static str,
    pub arity: u8,
    pub f: fn(Vec<f64>) -> f64,
    pub precedence: i32,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Function {}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}/{}({})", self.name, self.arity, self.precedence)
    }
}

impl Function {
    pub fn prioritize(&mut self, priority: i32) -> Self {
        self.precedence += priority;
        *self
    }
    pub fn was_spaced(&self) -> bool {
        let spaces = self.precedence / PRIORITY_SPACE;
        spaces % 2 == 0
    }
}

pub const COS: Function = Function {
    name: "cos",
    arity: 1,
    f: |x| x[0].cos(),
    precedence: PRIORITY_SPACE + 2,
};

pub const SIN: Function = Function {
    name: "sin",
    arity: 1,
    f: |x| x[0].sin(),
    precedence: PRIORITY_SPACE + 2,
};

pub const TAN: Function = Function {
    name: "tan",
    arity: 1,
    f: |x| x[0].tan(),
    precedence: PRIORITY_SPACE + 2,
};

pub const EXP: Function = Function {
    name: "exp",
    arity: 1,
    f: |x| x[0].exp(),
    precedence: PRIORITY_SPACE + 2,
};

pub const LN: Function = Function {
    name: "ln",
    arity: 1,
    f: |x| x[0].ln(),
    precedence: PRIORITY_SPACE + 2,
};

pub const LOG2: Function = Function {
    name: "log2",
    arity: 1,
    f: |x| x[0].log2(),
    precedence: PRIORITY_SPACE + 2,
};

pub const ACOS: Function = Function {
    name: "acos",
    arity: 1,
    f: |x| x[0].acos(),
    precedence: PRIORITY_SPACE + 2,
};

pub const ASIN: Function = Function {
    name: "asin",
    arity: 1,
    f: |x| x[0].asin(),
    precedence: PRIORITY_SPACE + 2,
};

pub const ATAN: Function = Function {
    name: "atan",
    arity: 1,
    f: |x| x[0].atan(),
    precedence: PRIORITY_SPACE + 2,
};

pub const COSH: Function = Function {
    name: "cosh",
    arity: 1,
    f: |x| x[0].cosh(),
    precedence: PRIORITY_SPACE + 2,
};

pub const SINH: Function = Function {
    name: "sinh",
    arity: 1,
    f: |x| x[0].sinh(),
    precedence: PRIORITY_SPACE + 2,
};

pub const TANH: Function = Function {
    name: "tanh",
    arity: 1,
    f: |x| x[0].tanh(),
    precedence: PRIORITY_SPACE + 2,
};

pub const ACOSH: Function = Function {
    name: "acosh",
    arity: 1,
    f: |x| x[0].acosh(),
    precedence: PRIORITY_SPACE + 2,
};

pub const ASINH: Function = Function {
    name: "asinh",
    arity: 1,
    f: |x| x[0].asinh(),
    precedence: PRIORITY_SPACE + 2,
};

pub const ATANH: Function = Function {
    name: "atanh",
    arity: 1,
    f: |x| x[0].atanh(),
    precedence: PRIORITY_SPACE + 2,
};

pub const NEG: Function = Function {
    name: "-",
    arity: 1,
    f: |x| -x[0],
    precedence: PRIORITY_SPACE + 2,
};

pub const ADD: Function = Function {
    name: "+",
    arity: 2,
    f: |xs| xs[1] + xs[0],
    precedence: PRIORITY_SPACE + 4,
};

pub const SUB: Function = Function {
    name: "-",
    arity: 2,
    f: |xs| xs[1] - xs[0],
    precedence: PRIORITY_SPACE + 4,
};

pub const MUL: Function = Function {
    name: "*",
    arity: 2,
    f: |xs| xs[1] * xs[0],
    precedence: PRIORITY_SPACE + 5,
};

pub const DIV: Function = Function {
    name: "/",
    arity: 2,
    f: |xs| xs[1] / xs[0],
    precedence: PRIORITY_SPACE + 5,
};
// TODO: Make right-associative
pub const POW: Function = Function {
    name: "^",
    arity: 2,
    f: |xs| xs[1].powf(xs[0]),
    precedence: PRIORITY_SPACE + 6,
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::prioritize::PRIORITY_PAREN;

    #[test]
    fn _was_spaced() {
        let mut it = Function {
            precedence: 5 + PRIORITY_SPACE,
            ..ADD
        };
        assert_eq!(it.was_spaced(), false);
        it.prioritize(-PRIORITY_SPACE);
        assert_eq!(it.was_spaced(), true);
        it.prioritize(PRIORITY_PAREN);
        assert_eq!(it.was_spaced(), true);
    }
}
