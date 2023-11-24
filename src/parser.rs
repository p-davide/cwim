use crate::token::*;

struct ParseState<'a> {
    to_parse: &'a str,
    tokens: Vec<Token<'a>>,
}

pub fn parse(text: &str) -> Option<Vec<Token>> {
    let mut state = ParseState {
        to_parse: text,
        tokens: vec![],
    };
    while state.to_parse.len() != 0 {
        match parse_token(state.to_parse) {
            Some(token) => {
                state.tokens.push(token);
                state.to_parse = &state.to_parse[token.lexeme.len()..];
            }
            None => return None,
        }
    }
    Some(state.tokens)
}

fn parse_token(text: &str) -> Option<Token> {
    if let Some(c) = text.chars().nth(0) {
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
            ' ' => return parse_space(text),
            '\n' => return parse_newline(text),
            '[' => return parse_lbracket(text),
            ']' => return parse_rbracket(text),
            '(' => return parse_lparen(text),
            ')' => return parse_rparen(text),
            ',' => return parse_comma(text),
            ';' => return parse_semicolon(text),
            '#' => return parse_comment(text),
            _ => {}
        }
    }
    None
}

const SYMBOLS: &str = "!@$%^&*|\"';,./+-";

fn parse_char(c: char, ttype: TokenType, text: &str) -> Option<Token> {
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

fn parse_space(text: &str) -> Option<Token> {
    parse_char(' ', TokenType::Space, text)
}

fn parse_comma(text: &str) -> Option<Token> {
    parse_char(',', TokenType::Comma, text)
}

fn parse_semicolon(text: &str) -> Option<Token> {
    parse_char(';', TokenType::Semicolon, text)
}

fn parse_newline(text: &str) -> Option<Token> {
    parse_char('\n', TokenType::Newline, text)
}

fn parse_lbracket(text: &str) -> Option<Token> {
    parse_char('[', TokenType::LBracket, text)
}

fn parse_rbracket(text: &str) -> Option<Token> {
    parse_char(']', TokenType::RBracket, text)
}

fn parse_lparen(text: &str) -> Option<Token> {
    parse_char('(', TokenType::LParen, text)
}

fn parse_rparen(text: &str) -> Option<Token> {
    parse_char(')', TokenType::RParen, text)
}

fn parse_comment(text: &str) -> Option<Token> {
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

fn parse_number(text: &str) -> Option<Token> {
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

fn parse_identifier(text: &str) -> Option<Token> {
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

fn parse_binary<'a>(text: &'a str) -> Option<Token<'a>> {
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
