use crate::binary::*;
use crate::parser::*;
use crate::prioritize::*;
use crate::token::*;
use crate::unary::*;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(PartialEq, Clone)]
enum Expr {
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

fn understand(tokens: Vec<Prioritized<Token>>) -> Parsed<Vec<Expr>> {
    let mut result: Vec<Expr> = vec![];
    for tok in tokens {
        // TODO: multiline expressions
        if tok.t.ttype != TokenType::Newline {
            let expr = understand_one(tok)?;
            result.push(expr);
        }
    }
    Ok(result)
}

fn understand_one(tok: Prioritized<Token>) -> Parsed<Expr> {
    match tok.t.ttype {
        TokenType::Literal => match tok.t.lexeme.parse::<f64>() {
            Ok(n) => Ok(Expr::Literal(n)),
            Err(_) => Err(format!("failed to parse: '{}'", tok.t.lexeme)),
        },
        TokenType::Identifier => Ok(Expr::Variable(tok.t.lexeme.to_owned())),
        TokenType::Binary => match tok.t.lexeme {
            "+" => Ok(Expr::Binary(ADD.clone().prioritize(tok.priority))),
            "-" => Ok(Expr::Binary(SUB.clone().prioritize(tok.priority))),
            "*" => Ok(Expr::Binary(MUL.clone().prioritize(tok.priority))),
            "/" => Ok(Expr::Binary(DIV.clone().prioritize(tok.priority))),
            "^" => Ok(Expr::Binary(POW.clone().prioritize(tok.priority))),
            op => Err(format!("unknown binary operation '{}'", op)),
        },
        TokenType::LParen => Ok(Expr::LParen),
        TokenType::RParen => Ok(Expr::RParen),
        TokenType::Error => Err("received synthetic error".to_owned()),
        ttype => unimplemented!("{:?}", ttype),
    }
}

// -- precedence

fn shuntingyard(exprs: Vec<Expr>) -> Parsed<Vec<Expr>> {
    let mut result = vec![];
    let mut ops: Vec<Binary> = vec![];
    for expr in exprs {
        match expr {
            Expr::Literal(_) | Expr::Variable(_) => result.push(expr),
            Expr::Binary(b) => {
                while let Some(op) = ops.last() {
                    // NOTE: This assumes every operator is left-associative.
                    if b.precedence <= op.precedence {
                        result.push(Expr::Binary(ops.pop().ok_or("no expressions".to_owned())?))
                    } else {
                        break;
                    }
                }
                ops.push(b)
            }
            expr => unimplemented!("{:?}", expr),
        }
    }
    while let Some(op) = ops.pop() {
        result.push(Expr::Binary(op))
    }
    Ok(result)
}

fn eval(shunted: Vec<Expr>) -> crate::parser::Parsed<f64> {
    let mut stack = vec![];
    for expr in shunted {
        match expr {
            Expr::Literal(n) => stack.push(n),
            Expr::Binary(b) => {
                let x = stack.pop().ok_or("empty x")?;
                let y = stack.pop().ok_or("empty y")?;
                let f = b.f;
                stack.push(f(x, y));
            }
            _ => unimplemented!(),
        }
    }
    stack.pop().ok_or("empty stack".to_owned())
}

pub fn run(text: &str) -> Parsed<f64> {
    let tks = parse(text)?;
    let parens = prioritize(tks);
    let exprs = understand(parens)?;
    let s = shuntingyard(exprs)?;
    eval(s)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn _shuntingyard() {
        assert_eq!(
            shuntingyard(vec![
                Expr::Literal(234.0),
                Expr::Binary(MUL),
                Expr::Literal(5.0),
            ]),
            Ok(vec![
                Expr::Literal(234.0 ),
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
            Ok(vec![
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
            Ok(vec![
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

    #[test]
    fn _understand() {
        let parsed = parse("234*5+7*8-18^3").expect("didn't parse");
        let ordered = prioritize(parsed);
        assert_eq!(
            understand(ordered),
            Ok(vec![
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
            ])
        )
    }
}
