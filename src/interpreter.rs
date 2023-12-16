use crate::function::*;
use crate::parser::*;
use crate::prioritize::*;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Literal(f64),
    Function(Function),
    Variable(String),
    Error(String),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "{:?}", n),
            Expr::Function(g) => write!(f, "{:?}", g),
            Expr::Variable(n) => write!(f, "V{:?}", n),
            Expr::Error(msg) => write!(f, "Error: {:?}", msg),
        }
    }
}

fn shuntingyard(exprs: Vec<Expr>) -> Parsed<Vec<Expr>> {
    let mut result = vec![];
    let mut ops: Vec<Function> = vec![];
    for expr in exprs {
        match expr {
            Expr::Literal(_) | Expr::Variable(_) => result.push(expr),
            Expr::Function(b) => {
                while let Some(op) = ops.last() {
                    // NOTE: This assumes:
                    // - Every binary operator is left-associative.
                    // - Every unary operator is right-associative.
                    if b.arity == 2 && b.precedence <= op.precedence {
                        result.push(Expr::Function(
                            ops.pop().ok_or("no expressions".to_owned())?,
                        ))
                    } else {
                        break;
                    }
                }
                ops.push(b)
            }
            Expr::Error(msg) => return Err(msg),
        }
    }
    while let Some(op) = ops.pop() {
        result.push(Expr::Function(op))
    }
    Ok(result)
}

fn eval(shunted: Vec<Expr>, env: &mut crate::env::Env) -> Parsed<f64> {
    if shunted
        .iter()
        .rev()
        .skip(1)
        .any(|it| *it == Expr::Function(ASSIGN))
    {
        return Err(
            "There must be only one assignment, and its result can't be used as a value".to_owned(),
        );
    }
    let mut stack = vec![];
    let mut initializee: Option<String> = None;
    for expr in shunted {
        match expr {
            Expr::Literal(n) => stack.push(n),
            Expr::Function(fun) => {
                let mut xs = vec![];
                for i in 0..fun.arity {
                    match stack.pop() {
                        Some(n) => {
                            if fun == ASSIGN {
                                env.assign(
                                    initializee.unwrap(),
                                    crate::env::Variable::Value(n),
                                );
                                return Ok(n);
                            } else {
                                xs.push(n)
                            }
                        }
                        None => {
                            return Err(format!(
                                "expected {} arguments to {}, found {}",
                                fun.arity, fun.name, i
                            ))
                        }
                    }
                }
                let f = fun.f;
                stack.push(f(xs));
            }
            Expr::Variable(var) => initializee = Some(var),
            Expr::Error(msg) => return Err(msg),
        }
    }
    stack.pop().ok_or("empty stack".to_owned())
}

pub fn run(text: &str, env: &mut crate::env::Env) -> Parsed<f64> {
    let tks = parse(text)?;
    let parens = prioritize(tks, env)?;
    let s = shuntingyard(parens)?;
    eval(s, env)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::env::*;

    #[test]
    fn _eval_3() {
        assert_eq!(
            eval(
                vec![
                    Expr::Literal(2.),
                    Expr::Literal(4.),
                    Expr::Function(POW),
                    Expr::Literal(5.),
                    Expr::Function(MUL),
                    Expr::Literal(6.),
                    Expr::Literal(1.),
                    Expr::Literal(9.),
                    Expr::Function(POW),
                    Expr::Function(ADD),
                    Expr::Function(ADD),
                ],
                &mut Env::std()
            ),
            eval(
                vec![
                    Expr::Literal(2.),
                    Expr::Literal(4.),
                    Expr::Function(POW),
                    Expr::Literal(5.),
                    Expr::Function(MUL),
                    Expr::Literal(6.),
                    Expr::Function(ADD),
                    Expr::Literal(1.),
                    Expr::Literal(9.),
                    Expr::Function(POW),
                    Expr::Function(ADD),
                ],
                &mut Env::std()
            )
        );
    }
    #[test]
    fn _shuntingyard() {
        assert_eq!(
            shuntingyard(vec![
                Expr::Literal(234.0),
                Expr::Function(MUL),
                Expr::Literal(5.0),
            ]),
            Ok(vec![
                Expr::Literal(234.0),
                Expr::Literal(5.0),
                Expr::Function(MUL),
            ])
        );
    }

    #[test]
    fn _shuntingyard_2() {
        assert_eq!(
            shuntingyard(vec![
                Expr::Literal(234.0),
                Expr::Function(MUL),
                Expr::Literal(5.0),
                Expr::Function(ADD),
                Expr::Literal(7.0),
                Expr::Function(MUL),
                Expr::Literal(8.0),
            ]),
            Ok(vec![
                Expr::Literal(234.0),
                Expr::Literal(5.0),
                Expr::Function(MUL),
                Expr::Literal(7.0),
                Expr::Literal(8.0),
                Expr::Function(MUL),
                Expr::Function(ADD),
            ])
        );
    }

    // " -(6) * -(6)"
    #[test]
    #[allow(const_item_mutation)]
    fn _shuntingyard_4() {
        assert_eq!(
            shuntingyard(vec![
                Expr::Function(NEG),
                Expr::Literal(6.),
                Expr::Function(MUL.prioritize(-PRIORITY_SPACE)),
                Expr::Function(NEG),
                Expr::Literal(6.),
            ]),
            Ok(vec![
                Expr::Literal(6.),
                Expr::Function(NEG),
                Expr::Literal(6.),
                Expr::Function(NEG),
                Expr::Function(MUL.prioritize(-PRIORITY_SPACE)),
            ])
        )
    }

    #[test]
    fn _eval_1() {
        assert_eq!(
            eval(
                vec![Expr::Literal(2.), Expr::Literal(5.), Expr::Function(SUB),],
                &mut Env::std()
            ),
            Ok(-3.)
        );
    }

    // " -(6) * -(6)"
    #[test]
    #[allow(const_item_mutation)]
    fn _eval_2() {
        assert_eq!(
            eval(
                vec![
                    Expr::Literal(6.),
                    Expr::Function(NEG),
                    Expr::Literal(6.),
                    Expr::Function(NEG),
                    Expr::Function(MUL.prioritize(-PRIORITY_SPACE)),
                ],
                &mut Env::std()
            ),
            Ok(36.)
        );
    }
}
