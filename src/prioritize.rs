use crate::token::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Prioritized<T> {
    pub priority: i32,
    pub t: T,
}

impl std::fmt::Debug for Prioritized<Token<'_>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}'{}'", self.priority, self.t.lexeme)
    }
}

impl<T> Prioritized<T> {
    pub fn ignore(t: T) -> Self {
        Self { priority: 0, t: t }
    }

    pub fn was_spaced(&self) -> bool {
        (self.priority / PRIORITY_BRACKETS) % 2 != 0
    }
}

const PRIORITY_BRACKETS: i32 = 10;

pub fn prioritize(tokens: Vec<Token>) -> Vec<Prioritized<Token>> {
    let mut result: Vec<Prioritized<Token>> = vec![];
    let mut balance = 0;
    for tok in tokens {
        dbg!(result.clone());
        match tok.ttype {
            TokenType::LParen => balance += 2,
            TokenType::RParen => {
                balance -= 2;
                // account for " )"
                if balance % 2 == 1 {
                    balance += 1;
                }
            }
            TokenType::Literal => {
                if let Some(last) = result.last() {
                    // let last be a literal -2
                    match last.t.ttype {
                        // ' -2'
                        TokenType::Space => {
                            result
                                .pop()
                                .expect("we matched on last, it should be there");
                            if let Some(llast) = result.last() {
                                match llast.t.ttype {
                                    // '8 -2'
                                    TokenType::Literal => {
                                        result.push(Prioritized::ignore(Token {
                                            lexeme: &tok.lexeme[..1],
                                            ttype: TokenType::Binary,
                                        }));
                                        result.push(Prioritized::ignore(Token {
                                            lexeme: &tok.lexeme[1..],
                                            ttype: TokenType::Literal,
                                        }));
                                    }
                                    // '5* -2', ' -2', ...
                                    _ => {
                                        result.push(Prioritized::ignore(tok));
                                    }
                                }
                            }
                        }
                        // '5-2'
                        TokenType::Literal => {
                            result.push(Prioritized::ignore(Token {
                                lexeme: &tok.lexeme[..1],
                                ttype: TokenType::Binary,
                            }));
                            result.push(Prioritized::ignore(Token {
                                lexeme: &tok.lexeme[1..],
                                ttype: TokenType::Literal,
                            }));
                        }
                        _ => {
                            result.push(Prioritized::ignore(tok));
                        }
                    }
                } else {
                    result.push(Prioritized::ignore(tok));
                }
            }
            TokenType::Binary => {
                let mut bin = Prioritized {
                    priority: PRIORITY_BRACKETS * balance,
                    t: tok,
                };
                if let Some(space) = result.last() {
                    if space.t.ttype == TokenType::Space {
                        bin.priority -= PRIORITY_BRACKETS;
                        result.pop().expect("result.last() was checked?");
                    }
                } else {
                    // The expression starts with a -, so we emit a 0
                    result.push(Prioritized::ignore(Token {
                        ttype: TokenType::Literal,
                        lexeme: "0",
                    }))
                }
                result.push(bin);
            }
            TokenType::Space => {
                if let Some(bin) = result.last_mut() {
                    if bin.t.ttype == TokenType::Binary && !bin.was_spaced() {
                        bin.priority -= PRIORITY_BRACKETS;
                    } else if bin.t.ttype != TokenType::Binary {
                        result.push(Prioritized::ignore(tok));
                    }
                }
            }
            tt => unimplemented!("{:?}", tt),
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _simple_priority() {
        let it @ [n, minus, m] = [
            Token {
                ttype: TokenType::Literal,
                lexeme: "8",
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "-",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "9",
            },
        ];
        assert_eq!(
            prioritize(Vec::from(it)),
            vec![
                Prioritized { priority: 0, t: n },
                Prioritized {
                    priority: 0,
                    t: minus
                },
                Prioritized { priority: 0, t: m },
            ]
        );
    }

    #[test]
    fn _with_spaces() {
        let it @ [n, _, minus, _, m] = [
            Token {
                ttype: TokenType::Literal,
                lexeme: "8",
            },
            Token {
                ttype: TokenType::Space,
                lexeme: " ",
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "-",
            },
            Token {
                ttype: TokenType::Space,
                lexeme: " ",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "9",
            },
        ];
        assert_eq!(
            prioritize(Vec::from(it)),
            vec![
                Prioritized { priority: 0, t: n },
                Prioritized {
                    priority: -PRIORITY_BRACKETS,
                    t: minus
                },
                Prioritized { priority: 0, t: m },
            ]
        );
    }

    #[test]
    fn _b() {
        let it @ [_lp, five, plus, _, msix, _rp, _mseven] = [
            Token {
                ttype: TokenType::LParen,
                lexeme: "(",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "5",
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "+",
            },
            Token {
                ttype: TokenType::Space,
                lexeme: " ",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "-6",
            },
            Token {
                ttype: TokenType::RParen,
                lexeme: ")",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "-7",
            },
        ];
        assert_eq!(
            prioritize(Vec::from(it)),
            vec![
                Prioritized::ignore(five),
                Prioritized {
                    priority: PRIORITY_BRACKETS,
                    t: plus
                },
                Prioritized::ignore(msix),
                Prioritized {
                    priority: 0,
                    t: Token {
                        ttype: TokenType::Binary,
                        lexeme: "-"
                    }
                },
                Prioritized {
                    priority: 0,
                    t: Token {
                        ttype: TokenType::Literal,
                        lexeme: "7"
                    }
                },
            ]
        )
    }

    #[test]
    fn _c() {
        let it @ [neg, _lp, five, plus, _, msix, _rp, _mseven] = [
            Token {
                ttype: TokenType::Binary,
                lexeme: "-",
            },
            Token {
                ttype: TokenType::LParen,
                lexeme: "(",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "5",
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "+",
            },
            Token {
                ttype: TokenType::Space,
                lexeme: " ",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "-6",
            },
            Token {
                ttype: TokenType::RParen,
                lexeme: ")",
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "-7",
            },
        ];
        assert_eq!(
            prioritize(Vec::from(it)),
            vec![
                Prioritized {
                    priority: 0,
                    t: Token {
                        ttype: TokenType::Literal,
                        lexeme: "0",
                    }
                },
                Prioritized {
                    priority: 0,
                    t: neg,
                },
                Prioritized::ignore(five),
                Prioritized {
                    priority: PRIORITY_BRACKETS,
                    t: plus,
                },
                Prioritized::ignore(msix),
                Prioritized {
                    priority: 0,
                    t: Token {
                        ttype: TokenType::Binary,
                        lexeme: "-",
                    }
                },
                Prioritized {
                    priority: 0,
                    t: Token {
                        ttype: TokenType::Literal,
                        lexeme: "7",
                    }
                },
            ]
        )
    }
}
