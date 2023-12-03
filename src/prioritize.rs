use crate::function::*;
use crate::interpreter::Expr;
use crate::token::*;

pub const PRIORITY_SPACE: i32 = 10;
pub const PRIORITY_PAREN: i32 = 2 * PRIORITY_SPACE;

pub fn prioritize(tokens: Vec<Token>) -> Vec<Expr> {
    let mut stack: Vec<Option<Expr>> = vec![];
    let mut balance = 0;
    let binary = |f: Function, balance: i32| (f.clone().prioritize(PRIORITY_SPACE * balance));
    let from_lexeme = |l: &str, balance: i32| match l {
        "+" => binary(ADD, balance),
        "-" => binary(SUB, balance),
        "*" => binary(MUL, balance),
        "/" => binary(DIV, balance),
        "^" => binary(POW, balance),
        _ => unimplemented!("binary operation '{}'", l),
    };
    for tok in tokens {
        dbg!(stack.clone());
        match tok.ttype {
            TokenType::LParen => balance += 2,
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
                            if let Some(llast) = stack.last() {
                                match llast {
                                    // '8 -2'
                                    Some(Expr::Literal(_)) => {
                                        stack.push(Some(Expr::Function(
                                            binary(ADD, balance).prioritize(-PRIORITY_SPACE),
                                        )));
                                        stack.push(Some(Expr::Literal(n)))
                                    }
                                    // '5* -2', ' -2', ...
                                    _ => stack.push(Some(Expr::Literal(n))),
                                }
                            }
                        }
                        // '5-2'
                        Some(Expr::Literal(_)) => {
                            stack.push(Some(Expr::Function(binary(ADD, balance))));
                            stack.push(Some(Expr::Literal(n)));
                        }
                        _ => {
                            stack.push(Some(Expr::Literal(n)));
                        }
                    }
                } else {
                    stack.push(Some(Expr::Literal(n)));
                }
            }
            TokenType::Identifier => match tok.lexeme {
                "cos" => stack.push(Some(Expr::Function(
                    COS.clone().prioritize(PRIORITY_SPACE * balance),
                ))),
                tt => unimplemented!("{:?}", tt),
            },
            TokenType::Binary => {
                let mut bin = from_lexeme(tok.lexeme, balance);
                match stack.last() {
                    None => stack.push(Some(Expr::Function(
                        NEG.clone().prioritize(PRIORITY_SPACE * balance),
                    ))),
                    Some(None) => {
                        bin.precedence -= PRIORITY_SPACE;
                        stack.pop().expect("result.last() was checked?");
                        if tok.lexeme == "-" {
                            match stack.last() {
                                None | Some(Some(Expr::Function(_))) => {
                                    stack.push(Some(Expr::Function(
                                        NEG.clone().prioritize(PRIORITY_SPACE * balance),
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
            }
            TokenType::Space => {
                if let Some(Some(Expr::Function(bin))) = stack.last_mut() {
                    if bin.arity == 2 && !bin.was_spaced() {
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
    for space_or_expr in stack {
        if let Some(expr) = space_or_expr {
            result.push(expr);
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _simple_priority() {
        let it = [
            Token::lit(8., "8."),
            Token::bin("-"),
            Token::lit(9., "9"),
        ];
        assert_eq!(
            prioritize(Vec::from(it)),
            vec![Expr::Literal(8.), Expr::Function(SUB), Expr::Literal(9.),]
        );
    }

    #[test]
    fn _with_spaces() {
        let it = [
            Token::lit(8., "8."),
            Token::space(),
            Token::bin("-"),
            Token::space(),
            Token::lit(9., "9"),
        ];
        assert_eq!(
            prioritize(Vec::from(it)),
            vec![
                Expr::Literal(8.),
                Expr::Function(SUB.prioritize(-PRIORITY_SPACE)),
                Expr::Literal(9.),
            ]
        );
    }

    // (5+ -6)-7
    #[test]
    fn _parens_and_spaces() {
        let it = [
            Token::lparen(),
            Token::lit(5., "5"),
            Token::bin("+"),
            Token::space(),
            Token::lit(-6., "-6"),
            Token::rparen(),
            Token::lit(-7., "-7"),
        ];
        assert_eq!(
            prioritize(Vec::from(it)), // (5+ -6)-7
            vec![
                Expr::Literal(5.),
                Expr::Function(ADD.prioritize(PRIORITY_PAREN - PRIORITY_SPACE)),
                Expr::Literal(-6.),
                Expr::Function(ADD),
                Expr::Literal(-7.),
            ]
        )
    }

    // -(5+ -6)-7
    #[test]
    fn _neg_parens_and_spaces() {
        let it = [
            Token::bin("-"),
            Token::lparen(),
            Token::lit(5., "5"),
            Token::bin("+"),
            Token::space(),
            Token::lit(-6., "-6"),
            Token::rparen(),
            Token::lit(-7., "-7"),
        ];
        assert_eq!(
            prioritize(Vec::from(it)),
            vec![
                Expr::Function(NEG),
                Expr::Literal(5.),
                Expr::Function(ADD.prioritize(PRIORITY_SPACE)),
                Expr::Literal(-6.),
                Expr::Function(ADD),
                Expr::Literal(-7.),
            ]
        )
    }

    // " -(6) * -(6)"
    #[test]
    fn _nsix() {
        assert_eq!(
            prioritize(vec![
                Token::space(),
                Token::bin("-"),
                Token::lparen(),
                Token::lit(6., "6"),
                Token::rparen(),
                Token::space(),
                Token::bin("*"),
                Token::space(),
                Token::bin("-"),
                Token::lparen(),
                Token::lit(6., "6"),
                Token::rparen(),
            ]),
            vec![
                Expr::Function(NEG),
                Expr::Literal(6.),
                Expr::Function(MUL.prioritize(-PRIORITY_SPACE)),
                Expr::Function(NEG),
                Expr::Literal(6.),
            ]
        );
    }
}
