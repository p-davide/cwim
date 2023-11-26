use crate::binary::*;
use crate::token::*;
use crate::unary::*;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(PartialEq, Clone)]
pub enum Expr {
    Literal(f64),
    Binary(Binary),
    Variable(String),
    Unary(Unary),
    LParen,
    RParen,
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "L{:?}", n),
            Expr::Binary(n) => write!(f, "B{:?}", n),
            Expr::Variable(n) => write!(f, "V{:?}", n),
            Expr::Unary(n) => write!(f, "U{:?}", n),
            Expr::LParen => write!(f, "("),
            Expr::RParen => write!(f, ")"),
        }
    }
}

pub fn understand(tokens: Vec<Token>) -> Option<Vec<Expr>> {
    let mut result: Vec<Expr> = vec![];
    for tok in tokens {
        // TODO: multiline expressions
        if tok.ttype != TokenType::Newline {
            let expr = understand_one(tok)?;
            result.push(expr);
        }
    }
    Some(result)
}

#[test]
fn _understand() {
    let parsed = crate::parser::parse("234*5+7*8-18^3");
    let parenthesized = parsed.map(|p|crate::parser::parenthesize(p)).flatten();
    let actual = parenthesized.map(|p|understand(p)).flatten();
    assert_eq!(actual, Some(vec![
        Expr::LParen,
        Expr::Literal(234.),
        Expr::Binary(MUL),
        Expr::Literal(5.),
        Expr::Binary(ADD),
        Expr::Literal(7.),
        Expr::Binary(MUL),
        Expr::Literal(8.),
        Expr::Binary(SUB),
        Expr::Literal(18.),
        Expr::Binary(POW),
        Expr::Literal(3.),
        Expr::RParen,
    ]))
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
        TokenType::LParen => Some(Expr::LParen),
        TokenType::RParen => Some(Expr::RParen),
        TokenType::Error => None,
        ttype => unimplemented!("{:?}", ttype),
    }
}

// -- precedence

pub fn shuntingyard(exprs: Vec<Expr>) -> Option<Vec<Expr>> {
    let mut result = vec![];
    let mut ops: Vec<Binary> = vec![];
    let mut balance = 0;
    for expr in exprs {
        match expr {
            Expr::LParen => balance += 1,
            Expr::RParen => balance -= 1,
            Expr::Literal(_) | Expr::Variable(_) => result.push(expr),
            Expr::Binary(mut b) => {
                b.precedence += 20 * balance;
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
            expr => unimplemented!("{:?}",expr)
        }
    }
    while let Some(op) = ops.pop() {
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
