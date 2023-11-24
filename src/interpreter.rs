use crate::token::*;

#[derive(PartialEq)]
pub enum Expr {
    Literal(f64),
    Binary(Binary),
    Variable(String),
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "L{:?}", n),
            Expr::Binary(n) => write!(f, "B{:?}", n),
            Expr::Variable(n) => write!(f, "V{:?}", n),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Binary {
    pub name: &'static str,
    pub f: fn(f64, f64) -> f64,
    pub precedence: usize,
}

impl std::fmt::Debug for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}:{}]", self.name, self.precedence)
    }
}

pub const ADD: Binary = Binary {
    name: "+",
    f: |x, y| x + y,
    precedence: 4,
};

pub const SUB: Binary = Binary {
    name: "-",
    f: |x, y| x - y,
    precedence: 4,
};

pub const MUL: Binary = Binary {
    name: "*",
    f: |x, y| x * y,
    precedence: 5,
};

pub const DIV: Binary = Binary {
    name: "/",
    f: |x, y| x / y,
    precedence: 5,
};

pub const POW: Binary = Binary {
    name: "^",
    f: |x, y| x.powf(y),
    precedence: 6,
};

pub fn understand(tokens: Vec<Token>) -> Option<Vec<Expr>> {
    let mut result: Vec<Expr> = vec![];
    for tok in tokens {
        match understand_one(tok) {
            Some(x) => result.push(x),
            None => return None,
        }
    }
    Some(result)
}

fn understand_one(tok: Token) -> Option<Expr> {
    match tok.ttype {
        TokenType::Literal => match tok.lexeme.parse::<f64>() {
            Ok(n) => Some(Expr::Literal(n)),
            Err(_) => None,
        },
        TokenType::Identifier => Some(Expr::Variable(tok.lexeme.to_owned())),
        TokenType::Binary => match tok.lexeme {
            "+" => Some(Expr::Binary(ADD)),
            "-" => Some(Expr::Binary(SUB)),
            "*" => Some(Expr::Binary(MUL)),
            "/" => Some(Expr::Binary(DIV)),
            "^" => Some(Expr::Binary(POW)),
            _ => None,
        },
        _ => unimplemented!(),
    }
}

// -- precedence

pub fn shuntingyard(exprs: Vec<Expr>) -> Option<Vec<Expr>> {
    let mut result = vec![];
    let mut ops: Vec<Binary> = vec![];
    for expr in exprs {
        match expr {
            Expr::Literal(_) | Expr::Variable(_) => result.push(expr),
            Expr::Binary(b) => {
                while let Some(op) = ops.last() {
                    // NOTE: This assumes every operator is left-associative.
                    if b.precedence <= op.precedence {
                        result.push(Expr::Binary(ops.pop()?))
                    } else {
                        break;
                    }
                }
                ops.push(b)
            }
        }
    }
    ops.reverse();
    for op in ops {
        result.push(Expr::Binary(op))
    }
    Some(result)
}

#[test]
fn _shuntingyard() {
    assert_eq!(
        shuntingyard(vec![
            Expr::Literal(234.0),
            Expr::Binary(MUL),
            Expr::Literal(5.0),
        ]),
        Some(vec![
            Expr::Literal(234.0),
            Expr::Literal(5.0),
            Expr::Binary(MUL),
        ])
    );
}
#[test]
fn _shuntingyard_2() {
    assert_eq!(
        shuntingyard(vec![
            Expr::Literal(234.0),
            Expr::Binary(MUL),
            Expr::Literal(5.0),
            Expr::Binary(ADD),
            Expr::Literal(7.0),
            Expr::Binary(MUL),
            Expr::Literal(8.0),
        ]),
        Some(vec![
            Expr::Literal(234.0),
            Expr::Literal(5.0),
            Expr::Binary(MUL),
            Expr::Literal(7.0),
            Expr::Literal(8.0),
            Expr::Binary(MUL),
            Expr::Binary(ADD),
        ])
    );
}

#[test]
fn _shuntingyard_3() {
    assert_eq!(
        shuntingyard(vec![
            Expr::Literal(2.0),
            Expr::Binary(POW),
            Expr::Literal(4.0),
            Expr::Binary(MUL),
            Expr::Literal(5.0),
            Expr::Binary(ADD),
            Expr::Literal(6.0),
            Expr::Binary(ADD),
            Expr::Literal(1.0),
            Expr::Binary(POW),
            Expr::Literal(9.0),
        ]),
        Some(vec![
            Expr::Literal(2.0),
            Expr::Literal(4.0),
            Expr::Binary(POW),
            Expr::Literal(5.0),
            Expr::Binary(MUL),
            Expr::Literal(6.0),
            Expr::Binary(ADD),
            Expr::Literal(1.0),
            Expr::Literal(9.0),
            Expr::Binary(POW),
            Expr::Binary(ADD),
        ])
    );
}

// -- eval

pub fn eval(shunted: Vec<Expr>) -> Option<f64> {
    let mut stack = vec![];
    for expr in shunted {
        match expr {
            Expr::Literal(n) => stack.push(n),
            Expr::Binary(b) => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                let f = b.f;
                stack.push(f(x, y));
            }
            _ => unimplemented!(),
        }
    }
    stack.pop()
}
