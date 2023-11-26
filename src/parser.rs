use crate::token::*;

struct ParseState<'a> {
    to_parse: &'a str,
    tokens: Vec<Token<'a>>,
}

pub type Parsed<T> = Option<T>;

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
            TokenType::Space => if !ignore_space {
                result.push(tok);
            } else {
                ignore_space = false;
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
    let actual = parsed.map(|ts|parenthesize(ts)).flatten();
    assert_eq!(actual, Some(vec![
        Token { ttype: TokenType::LParen , lexeme: "" },
        Token { ttype: TokenType::Literal , lexeme: "234" },
        Token { ttype: TokenType::Binary , lexeme: "*" },
        Token { ttype: TokenType::Literal , lexeme: "5" },
        Token { ttype: TokenType::Binary , lexeme: "+" },
        Token { ttype: TokenType::Literal , lexeme: "7" },
        Token { ttype: TokenType::Binary , lexeme: "*" },
        Token { ttype: TokenType::Literal , lexeme: "8" },
        Token { ttype: TokenType::Binary , lexeme: "-" },
        Token { ttype: TokenType::Literal , lexeme: "18" },
        Token { ttype: TokenType::Binary , lexeme: "^" },
        Token { ttype: TokenType::Literal , lexeme: "3" },
        Token { ttype: TokenType::RParen , lexeme: "" },
    ]));
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
    assert_eq!(parse("2 *2"), Some(vec![
        Token { ttype: TokenType::Literal, lexeme: "2" },
        Token { ttype: TokenType::Space, lexeme: " " },
        Token { ttype: TokenType::Binary, lexeme: "*" },
        Token { ttype: TokenType::Literal, lexeme: "2" },
    ]))
}

#[test]
fn _paren_token() {
    let parsed = parse("2 *2");
    let actual = parsed.map(|ts|parenthesize(ts)).flatten();
    assert_eq!(actual, Some(vec![
        Token { ttype: TokenType::LParen, lexeme: "" },
        Token { ttype: TokenType::Literal, lexeme: "2" },
        Token { ttype: TokenType::RParen, lexeme: "" },
        Token { ttype: TokenType::Binary, lexeme: "*" },
        Token { ttype: TokenType::LParen, lexeme: "" },
        Token { ttype: TokenType::Literal, lexeme: "2" },
        Token { ttype: TokenType::RParen, lexeme: "" },
    ]));
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
