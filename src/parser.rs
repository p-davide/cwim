use crate::token::*;

pub type Parsed<T> = Result<T, String>;

pub fn parse(text: &str) -> Parsed<Vec<Token>> {
    let mut to_parse = text;
    let mut tokens = vec![];
    while to_parse.len() != 0 {
        let token = parse_token(to_parse)?;
        if token.ttype == TokenType::Error {
            return Err("Received synthetic error".to_owned());
        }
        tokens.push(token);
        to_parse = &to_parse[token.lexeme.len()..];
    }
    Ok(tokens)
}

fn parse_token(text: &str) -> Parsed<Token> {
    let c = text.chars().nth(0).ok_or("Tried to parse empty token")?;
    if c.is_ascii_digit() {
        return parse_number(text);
    }
    if c.is_ascii_alphabetic() {
        return parse_identifier(text);
    }
    match c {
        '-' => {
            if let Ok(t) = parse_number(text) {
                Ok(t)
            } else {
                parse_binary(text)
            }
        }
        ' ' => parse_space(text),
        '\n' => parse_newline(text),
        '[' => parse_lbracket(text),
        ']' => parse_rbracket(text),
        '(' => parse_lparen(text),
        ')' => parse_rparen(text),
        ',' => parse_comma(text),
        ';' => parse_semicolon(text),
        '#' => parse_comment(text),
        _ => {
            if SYMBOLS.contains(c) {
                parse_binary(text)
            } else {
                Err(format!("Can't parse '{}'", c))
            }
        }
    }
}

pub const SYMBOLS: &str = "!@$%^&*|\"';,./+-";

fn parse_char(expected: char, ttype: TokenType, text: &str) -> Parsed<Token> {
    let actual = text.chars().nth(0).ok_or("Tried to parse empty token")?;
    if expected == actual {
        let token = Token {
            ttype: ttype,
            lexeme: &text[..1],
        };
        Ok(token)
    } else {
        Err(format!("found: {}, expected: {}", actual, expected))
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
            return Ok(token);
        } else {
            l = l + 1;
        }
    }
    if l == 0 {
        Err("empty comment".to_owned())
    } else {
        let token = Token {
            ttype: TokenType::Comment,
            lexeme: &text[..l],
        };
        Ok(token)
    }
}

fn parse_number(text: &str) -> Parsed<Token> {
    let mut l: usize = 0;
    for c in text.chars() {
        if c.is_ascii_digit() || c == '.' || (l == 0 && c == '-') {
            l += 1;
        } else {
            break;
        }
    }
    let lexeme = &text[..l];
    if lexeme == "-" {
        return Err("minus sign not part of negative number".to_owned());
    }
    if l == 0 {
        Err("empty number".to_owned())
    } else {
        let token = Token {
            ttype: TokenType::Literal,
            lexeme: lexeme,
        };
        Ok(token)
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
        Err("empty identifier".to_owned())
    } else {
        let token = Token {
            ttype: TokenType::Identifier,
            lexeme: &text[..l],
        };
        Ok(token)
    }
}

fn parse_binary<'a>(text: &'a str) -> Parsed<Token<'a>> {
    let actual = text.chars().nth(0).ok_or("there should be a char here")?;
    if SYMBOLS.contains(actual) {
        let token = Token {
            ttype: TokenType::Binary,
            lexeme: &text[..1],
        };
        Ok(token)
    } else {
        Err(format!("expected binary, found: {}", actual))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _parse() {
        let actual = parse("234*5+7*8-18^3").map(|ts| ts.iter().map(|t| t.lexeme).collect());
        assert_eq!(
            actual,
            Ok(vec![
                "234", "*", "5", "+", "7", "*", "8", "-18", "^", "3",
            ])
        );
    }

    #[test]
    fn _a() {
        let to_parse = "-(5+6)";
        let actual = parse(to_parse).map(|ts| ts.iter().map(|t| t.lexeme).collect());
        assert_eq!(actual, Ok(vec!["-", "(", "5", "+", "6", ")"]));
    }

    #[test]
    fn _b() {
        let to_parse = "-1 +4";
        let actual = parse(to_parse).map(|ts| ts.iter().map(|t| t.lexeme).collect());
        assert_eq!(actual, Ok(vec!["-1", " ", "+", "4"]));
    }

    #[test]
    fn _c() {
        let to_parse = "-( -1 +4)";
        let actual = parse(to_parse).map(|ts| ts.iter().map(|t| t.lexeme).collect());
        assert_eq!(actual, Ok(vec!["-", "(", " ", "-1", " ", "+", "4", ")"]));
    }
}
