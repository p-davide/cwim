use crate::env::Env;
use crate::function::*;
use crate::parser::*;
use crate::prioritize::*;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Clone, PartialEq)]
pub enum Expr {
    Literal(f64),
    Function(Function),
    Variable(String, f64),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "{:?}", n),
            Expr::Function(g) => write!(f, "{:?}", g),
            Expr::Variable(name, coef) => write!(f, "V{:?}{:?}", coef, name),
        }
    }
}

fn shuntingyard(exprs: Vec<Expr>) -> Parsed<Vec<Expr>> {
    let mut result = vec![];
    let mut ops: Vec<Function> = vec![];
    for expr in exprs {
        match expr {
            Expr::Literal(_) | Expr::Variable(_, _) => result.push(expr),
            Expr::Function(b) if b.arity != 2 => ops.push(b),
            Expr::Function(b) => {
                while let Some(op) = ops.pop() {
                    // NOTE: This assumes:
                    // - Every binary operator is left-associative.
                    // - Every unary operator is right-associative.
                    if b.precedence >= op.precedence {
                        result.push(Expr::Function(op))
                    } else {
                        ops.push(op);
                        break;
                    }
                }
                ops.push(b)
            }
        }
    }
    while let Some(op) = ops.pop() {
        result.push(Expr::Function(op))
    }
    Ok(result)
}

fn eval(shunted: Vec<Expr>, env: &mut Env) -> Parsed<f64> {
    // if shunted
    //     .iter()
    //     .rev()
    //     .skip(1)
    //     .any(|it| *it == Expr::Function(ASSIGN))
    // {
    //     return Err(
    //         "There must be only one assignment, and its result can't be used as a value".to_owned(),
    //     );
    // }
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
                                return match initializee {
                                    Some(name) if stack.is_empty() => {
                                        env.assign(name.clone(), crate::env::Variable::Value(n));
                                        println!("{} = {}", name, n);
                                        Ok(n)
                                    }
                                    _ => Err(format!("No name found to the left of {}", n)),
                                };
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
            Expr::Variable(var, _) => {
                if let Some(name) = initializee {
                    return Err(format!(
                        "Tried to initialize {} and {} at the same time",
                        var, name
                    ));
                } else {
                    initializee = Some(var);
                }
            }
        }
    }
    stack.pop().ok_or("empty stack".to_owned())
}

pub fn run(text: &str, env: &mut Env) -> Parsed<f64> {
    let tks = parse(text)?;
    let stmts = tks.split(|t| t.ttype == crate::token::TokenType::Newline);
    let mut result = Err("nothing was calculated".to_owned());
    for stmt in stmts {
        let parens = prioritize(stmt.iter(), env)?;
        let s = shuntingyard(parens)?;
        result = eval(s, env);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

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
                &mut Env::prelude()
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
                &mut Env::prelude()
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
                Expr::Function(MUL.space()),
                Expr::Function(NEG),
                Expr::Literal(6.),
            ]),
            Ok(vec![
                Expr::Literal(6.),
                Expr::Function(NEG),
                Expr::Literal(6.),
                Expr::Function(NEG),
                Expr::Function(MUL.space()),
            ])
        )
    }

    #[test]
    fn _eval_1() {
        assert_eq!(
            eval(
                vec![Expr::Literal(2.), Expr::Literal(5.), Expr::Function(SUB),],
                &mut Env::prelude()
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
                    Expr::Function(MUL.space()),
                ],
                &mut Env::prelude()
            ),
            Ok(36.)
        );
    }
}
