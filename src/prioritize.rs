use crate::env::*;
use crate::function::*;
use crate::interpreter::Expr;
use crate::parser::Parsed;
use crate::token::*;

pub const PRIORITY_SPACE: i32 = 10;
pub const PRIORITY_PAREN: i32 = 2 * PRIORITY_SPACE;

pub fn prioritize(tokens: Vec<Token>, env: &crate::env::Env) -> Parsed<Vec<Expr>> {
    // Inside `stack`, `None` indicates a space.
    let mut stack: Vec<Option<Expr>> = vec![];
    let mut balance = 0;
    fn adjust(mut f: Function, balance: i32) -> Function {
        f.prioritize(PRIORITY_SPACE * balance)
    }

    for tok in tokens {
        match tok.ttype {
            TokenType::LParen => {
                // 2(3+5) -> 2*(3+5)
                let (spaces, last) = if let Some(None) = stack.last() {
                    stack.pop().unwrap();
                    (1, stack.last())
                } else {
                    (0, stack.last())
                };

                if let Some(Some(Expr::Literal(_))) = last {
                    stack.push(Some(Expr::Function(
                        adjust(MUL, balance).prioritize(-PRIORITY_SPACE * spaces),
                    )));
                }

                balance += 2
            }
            TokenType::RParen => {
                balance -= 2;
                if balance < -1 {
                    stack.push(Some(Expr::Error("unmatched )".to_owned())));
                }
                // account for " )"
                if balance % 2 == 1 {
                    balance += 1;
                }
            }
            TokenType::Literal(n) => {
                if let Some(last) = stack.last() {
                    // let n = -2
                    match last {
                        // ' -2'
                        None => {
                            stack.pop().unwrap();
                            if let Some(Some(Expr::Literal(m))) = stack.last() {
                                // 5 -3 -> 5 - 3
                                // 5 6 -> 5 * 6
                                if n < 0. {
                                    stack.push(Some(Expr::Function(
                                        adjust(ADD, balance).prioritize(-PRIORITY_SPACE),
                                    )))
                                } else {
                                    return Err(format!(
                                        "No operation specified for {} and {}",
                                        m, n
                                    ));
                                }
                            }
                        }
                        // '5-2'
                        Some(Expr::Literal(_)) => {
                            stack.push(Some(Expr::Function(adjust(ADD, balance))));
                        }
                        _ => {}
                    }
                }
                stack.push(Some(Expr::Literal(n)));
            }
            TokenType::Identifier => match env.expr(tok.lexeme, Arity::Unary) {
                expr @ (Ok(Expr::Function(_)) | Ok(Expr::Literal(_))) => {
                    let (spaces, last) = if let Some(None) = stack.last() {
                        stack.pop().unwrap();
                        (1, stack.last())
                    } else {
                        (0, stack.last())
                    };

                    if let Some(Some(Expr::Literal(_))) = last {
                        stack.push(Some(Expr::Function(
                            adjust(MUL, balance).prioritize(-PRIORITY_SPACE * spaces),
                        )));
                    }
                    stack.push(Some(expr.unwrap()));
                }
                _ => stack.push(Some(Expr::Variable(tok.lexeme.to_owned()))),
            },
            TokenType::Symbol => {
                let expr = env.expr(tok.lexeme, Arity::Binary);
                if let Ok(Expr::Function(f)) = expr {
                    let mut bin = adjust(f, balance);

                    match stack.last() {
                        None => {
                            if let Expr::Function(g) = env.expr(tok.lexeme, Arity::Unary)? {
                                stack.push(Some(Expr::Function(
                                    g.clone().prioritize(PRIORITY_SPACE * balance),
                                )));
                            }
                        }
                        Some(None) => {
                            bin.precedence -= PRIORITY_SPACE;
                            stack.pop().expect("result.last() was checked?");
                            if let Ok(Expr::Function(g)) = env.expr(tok.lexeme, Arity::Unary) {
                                match stack.last() {
                                    None | Some(Some(Expr::Function(_))) => {
                                        stack.push(Some(Expr::Function(
                                            g.clone().prioritize(PRIORITY_SPACE * balance),
                                        )))
                                    }
                                    Some(None) => unreachable!("two space tokens in a row"),
                                    _ => stack.push(Some(Expr::Function(bin))),
                                }
                            } else {
                                stack.push(Some(Expr::Function(bin)));
                            }
                        }
                        // Transform SUB into NEG
                        Some(Some(Expr::Function(_))) => {
                            if tok.lexeme == "-" {
                                stack.push(Some(Expr::Function(
                                    NEG.clone().prioritize(PRIORITY_SPACE * balance),
                                )));
                            }
                        }
                        _ => stack.push(Some(Expr::Function(bin))),
                    }
                } else {
                    stack.push(Some(Expr::Error(format!(
                        "no function named '{}'",
                        tok.lexeme
                    ))));
                };
            }
            TokenType::Space => {
                if let Some(Some(Expr::Function(bin))) = stack.last_mut() {
                    if bin.arity <= 2 && !bin.was_spaced() {
                        bin.prioritize(-PRIORITY_SPACE);
                    } else {
                        stack.push(None);
                    }
                } else {
                    stack.push(None);
                }
            }
            TokenType::Comment => {
                break;
            }
            tt => unimplemented!("{:?}", tt),
        }
    }
    let mut result = vec![];
    // Remove any leftover `None`s (spaces)
    for expr in stack.into_iter().flatten() {
        result.push(expr);
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _simple_priority() {
        let it = [Token::lit(8., "8."), Token::sym("-"), Token::lit(9., "9")];
        assert_eq!(
            prioritize(Vec::from(it), &Env::std()),
            Ok(vec![
                Expr::Literal(8.),
                Expr::Function(SUB),
                Expr::Literal(9.),
            ])
        );
    }

    #[test]
    #[allow(const_item_mutation)]
    fn _with_spaces() {
        let it = [
            Token::lit(8., "8."),
            Token::space(),
            Token::sym("-"),
            Token::space(),
            Token::lit(9., "9"),
        ];
        assert_eq!(
            prioritize(Vec::from(it), &Env::std()),
            Ok(vec![
                Expr::Literal(8.),
                Expr::Function(SUB.prioritize(-PRIORITY_SPACE)),
                Expr::Literal(9.),
            ])
        );
    }

    // (5+ -6)-7
    #[test]
    #[allow(const_item_mutation)]
    fn _parens_and_spaces() {
        let it = [
            Token::lparen(),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::space(),
            Token::lit(-6., "-6"),
            Token::rparen(),
            Token::lit(-7., "-7"),
        ];
        assert_eq!(
            prioritize(Vec::from(it), &Env::std()), // (5+ -6)-7
            Ok(vec![
                Expr::Literal(5.),
                Expr::Function(ADD.prioritize(PRIORITY_PAREN - PRIORITY_SPACE)),
                Expr::Literal(-6.),
                Expr::Function(ADD),
                Expr::Literal(-7.),
            ])
        )
    }

    // -(5+ -6)-7
    #[test]
    #[allow(const_item_mutation)]
    fn _neg_parens_and_spaces() {
        let it = [
            Token::sym("-"),
            Token::lparen(),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::space(),
            Token::lit(-6., "-6"),
            Token::rparen(),
            Token::lit(-7., "-7"),
        ];
        assert_eq!(
            prioritize(Vec::from(it), &Env::std()),
            Ok(vec![
                Expr::Function(NEG),
                Expr::Literal(5.),
                Expr::Function(ADD.prioritize(PRIORITY_SPACE)),
                Expr::Literal(-6.),
                Expr::Function(ADD),
                Expr::Literal(-7.),
            ])
        )
    }

    // " -(6) * -(6)"
    #[test]
    #[allow(const_item_mutation)]
    fn _nsix() {
        assert_eq!(
            prioritize(
                vec![
                    Token::space(),
                    Token::sym("-"),
                    Token::lparen(),
                    Token::lit(6., "6"),
                    Token::rparen(),
                    Token::space(),
                    Token::sym("*"),
                    Token::space(),
                    Token::sym("-"),
                    Token::lparen(),
                    Token::lit(6., "6"),
                    Token::rparen(),
                ],
                &Env::std()
            ),
            Ok(vec![
                Expr::Function(NEG),
                Expr::Literal(6.),
                Expr::Function(MUL.prioritize(-PRIORITY_SPACE)),
                Expr::Function(NEG),
                Expr::Literal(6.),
            ])
        );
    }
}
