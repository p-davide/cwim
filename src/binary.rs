use std::fmt::Debug;
use std::fmt::Formatter;


#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Binary {
    pub name: &'static str,
    pub f: fn(f64, f64) -> f64,
    pub precedence: usize,
}

impl Debug for Binary {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}:{}]", self.name, self.precedence)
    }
}

pub const ADD: Binary = Binary {
    name: "+",
    f: |y, x| x + y,
    precedence: 14,
};

pub const SUB: Binary = Binary {
    name: "-",
    f: |y, x| x - y,
    precedence: 14,
};

pub const MUL: Binary = Binary {
    name: "*",
    f: |y, x| x * y,
    precedence: 15,
};

pub const DIV: Binary = Binary {
    name: "/",
    f: |y, x| x / y,
    precedence: 15,
};

// TODO: Make right-associative
pub const POW: Binary = Binary {
    name: "^",
    f: |y, x| x.powf(y),
    precedence: 16,
};
