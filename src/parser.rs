use num::Num;

use crate::{env::Env, number::Number, token::*};

pub type Parsed<T> = Result<T, String>;
type Expression<'a> = Vec<Token<'a>>;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt<'a> {
    Expr(Expression<'a>),
    Assignment(Expression<'a>, Expression<'a>),
}

impl<'a> Stmt<'a> {
    pub fn rhs(&self) -> &Expression<'a> {
        match self {
            Self::Assignment(_, it) => it,
            Self::Expr(it) => it,
        }
    }
}

pub fn stmt<'a>(text: &'a str, env: &Env) -> Parsed<Stmt<'a>> {
    let mut tokens = vec![];

    let mut sides = text.split('=');
    let mut lhs = sides.next().expect("no =?");
    let mut column = 1;
    while !lhs.is_empty() {
        let token = token(lhs, env, &mut column)?;
        tokens.push(token);
        lhs = &lhs[token.lexeme.len()..];
    }
    if let Some(mut rhs) = sides.next() {
        column += 1;
        let mut right_tokens = vec![];
        while !rhs.is_empty() {
            let token = token(rhs, env, &mut column)?;
            right_tokens.push(token);
            rhs = &rhs[token.lexeme.len()..];
        }
        Ok(Stmt::Assignment(tokens, right_tokens))
    } else {
        Ok(Stmt::Expr(tokens))
    }
}

fn token<'a>(text: &'a str, _env: &Env, column: &mut usize) -> Parsed<Token<'a>> {
    match text.chars().next().ok_or("Tried to parse empty token")? {
        c if c.is_ascii_digit() => number(text, column),
        c if c.is_ascii_alphabetic() => identifier(text, column),
        '-' => symbol(text, column),
        ' ' => spaces(text, column),
        '\n' => newline(text, column),
        '[' => lbracket(text, column),
        ']' => rbracket(text, column),
        '(' => lparen(text, column),
        ')' => rparen(text, column),
        ',' => comma(text, column),
        ';' => semicolon(text, column),
        '#' => comment(text, column),
        c if SYMBOLS.contains(c) => symbol(text, column),
        c => Err(format!("Can't parse '{}'", c)),
    }
}

pub const SYMBOLS: &str = "!@$%^&*|\"';,./+-=";

fn char<'a>(
    expected: char,
    ttype: TokenType,
    text: &'a str,
    column: &mut usize,
) -> Parsed<Token<'a>> {
    let actual = text.chars().next().ok_or("Tried to parse empty token")?;
    if expected == actual {
        *column += 1;
        Ok(Token::new(ttype, &text[..1], *column - 1))
    } else {
        Err(format!("found: {}, expected: {}", actual, expected))
    }
}

fn spaces<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    let l = text.chars().take_while(|c| *c == ' ').count();
    if l == 0 {
        Err("empty space token".to_owned())
    } else {
        *column += l;
        Ok(Token::new(TokenType::Space, &text[..l], *column - l))
    }
}

fn comma<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    char(',', TokenType::Comma, text, column)
}

fn semicolon<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    char(';', TokenType::Semicolon, text, column)
}

fn newline<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    char('\n', TokenType::Newline, text, column)
}

fn lbracket<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    char('[', TokenType::LBracket, text, column)
}

fn rbracket<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    char(']', TokenType::RBracket, text, column)
}

fn lparen<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    char('(', TokenType::LParen, text, column)
}

fn rparen<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    char(')', TokenType::RParen, text, column)
}

fn comment<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    let l = text.chars().take_while(|c| *c != '\n').count();
    if l == 0 {
        Err("empty comment".to_owned())
    } else {
        *column += l;
        Ok(Token::new(TokenType::Comment, &text[..l], *column - l))
    }
}

fn number<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    let mut l: usize = 0;
    let mut h: usize = 0;
    let mut radix = 10;
    if &text[1..1] == "-" {
        h += 1;
    }
    if text[h..].len() > 2 && &text[h..h + 2] == "0x" {
        h += 2;
        radix = 16;
    }
    if text[h..].len() > 2 && &text[h..h + 2] == "0o" {
        h += 2;
        radix = 8;
    }
    if text[h..].len() > 2 && &text[h..h + 2] == "0b" {
        h += 2;
        radix = 2;
    }
    for c in text[h..].chars() {
        // Using c.is_digit(radix) can parse one valid digit as 2 valid ones
        // e.g. 0b112 is parsed as 0b11 2, which then evals to 6, which is
        // likely not what the user intended
        if ((radix <= 10 && c.is_ascii_digit()) || (radix == 16 && c.is_ascii_hexdigit()))
            || c == '.'
        {
            l += 1;
        } else {
            break;
        }
    }
    let lexeme = &text[..h + l];
    if lexeme == "-" {
        return Err("minus sign not part of negative number".to_owned());
    }
    let parsed = Number::from_str_radix(&lexeme[h..], radix);
    match parsed {
        Err(_) => Err(format!("failed to parse '{}' in base {}", lexeme, radix)),
        Ok(_) if l == 0 => Err("empty number".to_owned()),
        Ok(n) => {
            *column += l + h;
            Ok(Token::lit(n, lexeme, *column - l - h))
        }
    }
}

fn identifier<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    let mut l: usize = 0;
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            l += 1;
        } else {
            break;
        }
    }
    if l == 0 {
        return Err("empty identifier".to_owned());
    }
    *column += l;
    Ok(Token {
        ttype: TokenType::Identifier,
        lexeme: &text[..l],
        column: *column - l,
    })
}

fn symbol<'a>(text: &'a str, column: &mut usize) -> Parsed<Token<'a>> {
    let actual = text.chars().next().ok_or("there should be a char here")?;
    if SYMBOLS.contains(actual) {
        *column += 1;
        Ok(Token::sym(&text[..1], *column - 1))
    } else {
        Err(format!("expected binary, found: {}", actual))
    }
}

#[cfg(test)]
mod test {
    use crate::env;

    use super::*;

    fn test_expr(input: &str, expected_tokens: Vec<Token>) {
        let actual = stmt(input, &env::Env::prelude()).unwrap();
        let expected = Stmt::Expr(expected_tokens);
        assert_eq!(expected, actual);
    }

    fn _0b() {
        assert!(stmt("0b112", &env::Env::prelude()).is_err());
    }

    #[test]
    fn _0x() {
        test_expr("0xaC", vec![Token::lit(Number::Int(0xac), "0xaC", 1)]);
        test_expr("0x.", vec![Token::lit(Number::Flt(0.), "0x.", 1)]);
    }

    #[test]
    fn _z() {
        test_expr(
            "2 (+3+5)",
            vec![
                Token::lit(Number::Int(2), "2", 1),
                Token::space(2),
                Token::lparen(3),
                Token::sym("+", 4),
                Token::lit(Number::Int(3), "3", 5),
                Token::sym("+", 6),
                Token::lit(Number::Int(5), "5", 7),
                Token::rparen(8),
            ],
        )
    }

    #[test]
    fn _spaces() {
        test_expr("       ", vec![Token::new(TokenType::Space, "       ", 1)])
    }

    #[test]
    fn _assignment() {
        assert_eq!(
            stmt("x=6", &env::Env::prelude()).unwrap(),
            Stmt::Assignment(
                vec![Token::new(TokenType::Identifier, "x", 1)],
                vec![Token::lit(Number::Int(6), "6", 3)]
            )
        );
        test_expr(
            "7x+5y",
            vec![
                Token::lit(Number::Int(7), "7", 1),
                Token::new(TokenType::Identifier, "x", 2),
                Token::new(TokenType::Symbol, "+", 3),
                Token::lit(Number::Int(5), "5", 4),
                Token::new(TokenType::Identifier, "y", 5),
            ],
        )
    }

    #[test]
    fn _parse() {
        let expected = Ok(Stmt::Expr(vec![
            Token::lit(Number::Int(234), "234", 1),
            Token::sym("*", 4),
            Token::lit(Number::Int(5), "5", 5),
            Token::sym("+", 6),
            Token::lit(Number::Int(7), "7", 7),
            Token::sym("*", 8),
            Token::lit(Number::Int(8), "8", 9),
            Token::sym("-", 10),
            Token::lit(Number::Int(18), "18", 11),
            Token::sym("^", 13),
            Token::lit(Number::Int(3), "3", 14),
        ]));
        assert_eq!(stmt("234*5+7*8-18^3", &env::Env::prelude()), expected);
    }

    #[test]
    fn _a() {
        let to_parse = "-(5+6)";
        let expected = Ok(Stmt::Expr(vec![
            Token::sym("-", 1),
            Token::lparen(2),
            Token::lit(Number::Int(5), "5", 3),
            Token::sym("+", 4),
            Token::lit(Number::Int(6), "6", 5),
            Token::rparen(6),
        ]));
        assert_eq!(stmt(to_parse, &env::Env::prelude()), expected);
    }

    #[test]
    fn _b() {
        let to_parse = "-1 +4";
        let expected = Ok(Stmt::Expr(vec![
            Token::sym("-", 1),
            Token::lit(Number::Int(1), "1", 2),
            Token::space(3),
            Token::sym("+", 4),
            Token::lit(Number::Int(4), "4", 5),
        ]));
        assert_eq!(stmt(to_parse, &env::Env::prelude()), expected);
    }

    #[test]
    fn _d() {
        let to_parse = " -(6) * -(6)";
        assert_eq!(
            stmt(to_parse, &env::Env::prelude()),
            Ok(Stmt::Expr(vec![
                Token::space(1),
                Token::sym("-", 2),
                Token::lparen(3),
                Token::lit(Number::Int(6), "6", 4),
                Token::rparen(5),
                Token::space(6),
                Token::sym("*", 7),
                Token::space(8),
                Token::sym("-", 9),
                Token::lparen(10),
                Token::lit(Number::Int(6), "6", 11),
                Token::rparen(12),
            ]))
        );
    }
}
