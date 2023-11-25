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

impl Binary {
    pub fn minus_pred(&mut self, prec: usize) -> Self {
        Self {
            name: self.name,
            f: self.f,
            precedence: self.precedence - prec,
        }
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
