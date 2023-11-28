use crate::token::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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
    let mut result = vec![];
    let mut balance = 0;
    for tok in tokens {
        match tok.ttype {
            TokenType::LParen => balance += 2,
            TokenType::RParen => {
                balance -= 2;
                // account for " )"
                if balance % 2 == 1 {
                    balance += 1;
                }
            }
            TokenType::Literal | TokenType::Identifier => {
                result.push(Prioritized::ignore(tok));
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
