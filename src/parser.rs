use crate::token::*;

struct ParseState<'a> {
    to_parse: &'a str,
    tokens: Vec<Token<'a>>,
}

pub type Parsed<T> = Option<T>;

impl<'a> ParseState<'a> {
    fn str(&mut self, expected: &str, ttype: TokenType) -> Parsed<()> {
        let n = expected.len();
        if self.to_parse.starts_with(expected) {
            self.tokens.push(Token {
                ttype: ttype,
                lexeme: &self.to_parse[..n],
            });
            self.to_parse = &self.to_parse[n..];
            Some(())
        } else {
            None
        }
    }

    fn lp(&mut self) -> Parsed<()> {
        self.str("(", TokenType::LParen)
    }

    fn rp(&mut self) -> Parsed<()> {
        self.str(")", TokenType::RParen)
    }

    fn pat<F: Fn(char) -> bool>(&mut self, maybe_ttype: Option<TokenType>, pred: F) -> Parsed<()> {
        let trimmed = self.to_parse.trim_start_matches(pred);
        let n = self.to_parse.len() - trimmed.len();
        if trimmed == self.to_parse {
            None
        } else {
            if let Some(ttype) = maybe_ttype {
                self.tokens.push(Token {
                    ttype: ttype,
                    lexeme: &self.to_parse[..n],
                });
            }
            self.to_parse = trimmed;
            Some(())
        }
    }

    // lp space? lhs rhs* rp
    fn top(&mut self) -> Parsed<()> {
        self.lp()?;
        self.spaces();
        dbg!(&self.tokens);
        self.lhs()?;
        dbg!(&self.tokens);
        self.zero_plus_rhs()?;
        dbg!(&self.tokens);
        self.rp()
    }

    fn zero_plus_rhs(&mut self) -> Parsed<()> {
        while let Some(_) = self.rhs() {
            continue;
        }
        Some(())
    }

    fn spaces(&mut self) -> Parsed<()> {
        self.pat(Some(TokenType::Space), |c| c.is_ascii_whitespace())
    }

    // (space? bin space? side space?)
    fn rhs(&mut self) -> Parsed<()> {
        self.spaces();
        self.pat(Some(TokenType::Binary), |c| SYMBOLS.contains(c))?;
        self.spaces();
        self.lhs()?;
        self.spaces()
    }

    // lit | top
    fn lhs(&mut self) -> Parsed<()> {
        self.lit().or_else(||self.top())
    }

    fn lit(&mut self) -> Parsed<()> {
        self.pat(Some(TokenType::Literal), |c: char| c.is_ascii_digit())
    }
}

pub fn parse2(text: &str) -> Parsed<Vec<Token>> {
    let mut state = ParseState {
        to_parse: text,
        tokens: vec![],
    };
    state.top();
    Some(state.tokens)
}

pub fn parse(text: &str) -> Parsed<Vec<Token>> {
    let mut state = ParseState {
        to_parse: text,
        tokens: vec![],
    };
    while state.to_parse.len() != 0 {
        match parse_token(state.to_parse) {
            Some(token) => {
                if token.ttype == TokenType::Error {
                    return None;
                }
                state.tokens.push(token);
                state.to_parse = &state.to_parse[token.lexeme.len()..];
            }
            None => return None,
        }
    }
    Some(state.tokens)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Ordered<T> {
    priority: usize,
    t: T,
}

const PRIORITY_BRACKETS: usize = 10;

fn prioritize(tokens: Vec<Token>) -> Vec<Ordered<Token>> {
    let mut result = vec![];
    let mut balance = 0;
    let mut increase = false;
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
                result.push(Ordered {
                    priority: 0,
                    t: tok,
                })
            },
            TokenType::Binary => result.push(Ordered {
                priority: PRIORITY_BRACKETS * balance,
                t: tok,
            }),
            TokenType::Space => {
                if increase {
                    balance += 1
                } else {
                    balance -= 1
                };
                increase = !increase;
            }
            tt => unimplemented!("{:?}", tt),
        }
    }
    result
}

pub fn parenthesize(spaced: Vec<Token>) -> Option<Vec<Token>> {
    let mut result = vec![];
    let mut ignore_space = false;

    let lparen = Token {
        ttype: TokenType::LParen,
        lexeme: "",
    };
    let rparen = Token {
        ttype: TokenType::RParen,
        lexeme: "",
    };

    result.push(lparen);
    for tok in spaced {
        match tok.ttype {
            TokenType::Binary => {
                if result.last()?.ttype == TokenType::Space {
                    ignore_space = true;
                    result.pop();
                    result.push(rparen);
                    result.push(tok);
                    result.push(lparen);
                } else {
                    result.push(tok);
                }
            }
            TokenType::Space => {
                if !ignore_space {
                    result.push(tok);
                } else {
                    ignore_space = false;
                }
            }
            _ => {
                ignore_space = false;
                result.push(tok)
            }
        }
    }
    result.push(rparen);
    Some(result)
}

#[test]
fn _parenthesize() {
    let parsed = parse("234*5+7*8-18^3");
    let actual = parsed.map(|ts| parenthesize(ts)).flatten();
    assert_eq!(
        actual,
        Some(vec![
            Token {
                ttype: TokenType::LParen,
                lexeme: ""
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "234"
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "*"
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "5"
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "+"
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "7"
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "*"
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "8"
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "-"
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "18"
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "^"
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "3"
            },
            Token {
                ttype: TokenType::RParen,
                lexeme: ""
            },
        ])
    );
}

fn parse_token(text: &str) -> Parsed<Token> {
    let c = text.chars().nth(0)?;
    if c.is_ascii_digit() {
        return parse_number(text);
    }
    if SYMBOLS.contains(c) {
        return parse_binary(text);
    }
    if c.is_ascii_alphabetic() {
        return parse_identifier(text);
    }
    match c {
        ' ' => parse_space(text),
        '\n' => parse_newline(text),
        '[' => parse_lbracket(text),
        ']' => parse_rbracket(text),
        '(' => parse_lparen(text),
        ')' => parse_rparen(text),
        ',' => parse_comma(text),
        ';' => parse_semicolon(text),
        '#' => parse_comment(text),
        _ => None,
    }
}

#[test]
fn _parse_token() {
    assert_eq!(
        parse2("(2 *2)"),
        Some(vec![
            Token {
                ttype: TokenType::LParen,
                lexeme: "("
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "2"
            },
            Token {
                ttype: TokenType::Space,
                lexeme: " "
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "*"
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "2"
            },
            Token {
                ttype: TokenType::RParen,
                lexeme: ")"
            },
        ])
    )
}

#[test]
fn _paren_token() {
    let parsed = parse("2 *2");
    let actual = parsed.map(|ts| parenthesize(ts)).flatten();
    assert_eq!(
        actual,
        Some(vec![
            Token {
                ttype: TokenType::LParen,
                lexeme: ""
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "2"
            },
            Token {
                ttype: TokenType::RParen,
                lexeme: ""
            },
            Token {
                ttype: TokenType::Binary,
                lexeme: "*"
            },
            Token {
                ttype: TokenType::LParen,
                lexeme: ""
            },
            Token {
                ttype: TokenType::Literal,
                lexeme: "2"
            },
            Token {
                ttype: TokenType::RParen,
                lexeme: ""
            },
        ])
    );
}

const SYMBOLS: &str = "!@$%^&*|\"';,./+-";

fn parse_char(c: char, ttype: TokenType, text: &str) -> Parsed<Token> {
    if text.chars().nth(0) == Some(c) {
        let token = Token {
            ttype: ttype,
            lexeme: &text[..1],
        };
        Some(token)
    } else {
        None
    }
}

fn parse_space(text: &str) -> Parsed<Token> {
    parse_char(' ', TokenType::Space, text)
}

fn parse_comma(text: &str) -> Parsed<Token> {
    parse_char(',', TokenType::Comma, text)
}

fn parse_semicolon(text: &str) -> Parsed<Token> {
    parse_char(';', TokenType::Semicolon, text)
}

fn parse_newline(text: &str) -> Parsed<Token> {
    parse_char('\n', TokenType::Newline, text)
}

fn parse_lbracket(text: &str) -> Parsed<Token> {
    parse_char('[', TokenType::LBracket, text)
}

fn parse_rbracket(text: &str) -> Parsed<Token> {
    parse_char(']', TokenType::RBracket, text)
}

fn parse_lparen(text: &str) -> Parsed<Token> {
    parse_char('(', TokenType::LParen, text)
}

fn parse_rparen(text: &str) -> Parsed<Token> {
    parse_char(')', TokenType::RParen, text)
}

fn parse_comment(text: &str) -> Parsed<Token> {
    let mut l: usize = 0;
    for c in text.chars() {
        if c == '\n' {
            let token = Token {
                ttype: TokenType::Comment,
                lexeme: &text[..l],
            };
            return Some(token);
        } else {
            l = l + 1;
        }
    }
    if l == 0 {
        None
    } else {
        let token = Token {
            ttype: TokenType::Comment,
            lexeme: &text[..l],
        };
        Some(token)
    }
}

fn parse_number(text: &str) -> Parsed<Token> {
    let mut l: usize = 0;
    for c in text.chars() {
        if c.is_ascii_digit() {
            l += 1;
        } else {
            break;
        }
    }
    if l == 0 {
        None
    } else {
        let token = Token {
            ttype: TokenType::Literal,
            lexeme: &text[..l],
        };
        Some(token)
    }
}

fn parse_identifier(text: &str) -> Parsed<Token> {
    let mut l: usize = 0;
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            l += 1;
        } else {
            break;
        }
    }
    if l == 0 {
        None
    } else {
        let token = Token {
            ttype: TokenType::Identifier,
            lexeme: &text[..l],
        };
        Some(token)
    }
}

fn parse_binary<'a>(text: &'a str) -> Parsed<Token<'a>> {
    if SYMBOLS.contains(text.chars().nth(0).expect("there should be a char here")) {
        let token = Token {
            ttype: TokenType::Binary,
            lexeme: &text[..1],
        };
        Some(token)
    } else {
        None
    }
}
