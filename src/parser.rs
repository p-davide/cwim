use std::str::FromStr;

use num_traits::real::Real;

use crate::{env::Env, interpreter::Expr, token::*};

pub type Parsed<T> = Result<T, String>;
type Expression<'a> = Vec<Token<'a>>;

#[derive(Debug, PartialEq)]
pub enum Stmt<'a> {
    Expr(Expression<'a>),
    Assignment(String, Expression<'a>),
}

impl<'a> Stmt<'a> {
    pub fn rhs(&self) -> &Expression<'a> {
        match self {
            Self::Assignment(_, it) => it,
            Self::Expr(it) => it,
        }
    }
}

fn parse_assignee<N>(text: &str) -> Parsed<(usize, String)> {
    let mut i = 0;

    if let Ok(token) = parse_spaces::<N>(text) {
        i += token.lexeme.len();
    }

    let assignee = parse_identifier(&text[i..])?;
    i += assignee.len();

    if let Ok(token) = parse_spaces::<N>(&text[i..]) {
        i += token.lexeme.len();
    }

    parse_char::<N>('=', TokenType::Space, &text[i..])?;
    i += 1;

    if let Ok(token) = parse_spaces::<N>(&text[i..]) {
        i += token.lexeme.len();
    }

    Ok((i, assignee.to_owned()))
}

pub fn parse<'a, N: Real + FromStr>(text: &'a str, env: &Env<N>) -> Parsed<Stmt<'a>>
where
    <N as FromStr>::Err: std::fmt::Debug,
{
    let mut tokens = vec![];
    if let Ok((n, assignee)) = parse_assignee::<N>(text) {
        let mut to_parse = &text[n..];
        while !to_parse.is_empty() {
            let token = parse_token(to_parse, env)?;
            tokens.push(token);
            to_parse = &to_parse[token.lexeme.len()..];
        }
        Ok(Stmt::Assignment(assignee, tokens))
    } else {
        let mut to_parse = text;
        while !to_parse.is_empty() {
            let token = parse_token(to_parse, env)?;
            tokens.push(token);
            to_parse = &to_parse[token.lexeme.len()..];
        }
        Ok(Stmt::Expr(tokens))
    }
}

fn parse_token<'a, N: Real + FromStr>(text: &'a str, env: &Env<N>) -> Parsed<Token<'a>>
where
    <N as FromStr>::Err: std::fmt::Debug,
{
    let c = text.chars().next().ok_or("Tried to parse empty token")?;
    if c.is_ascii_digit() {
        return parse_number::<N>(text);
    }
    if c.is_ascii_alphabetic() {
        return parse_known_identifier(text, env);
    }
    match c {
        '-' => parse_symbol::<N>(text),
        ' ' => parse_spaces::<N>(text),
        '\n' => parse_newline::<N>(text),
        '[' => parse_lbracket::<N>(text),
        ']' => parse_rbracket::<N>(text),
        '(' => parse_lparen::<N>(text),
        ')' => parse_rparen::<N>(text),
        ',' => parse_comma::<N>(text),
        ';' => parse_semicolon::<N>(text),
        '#' => parse_comment::<N>(text),
        _ => {
            if SYMBOLS.contains(c) {
                parse_symbol::<N>(text)
            } else {
                Err(format!("Can't parse '{}'", c))
            }
        }
    }
}

pub const SYMBOLS: &str = "!@$%^&*|\"';,./+-=";

fn parse_char<N>(expected: char, ttype: TokenType, text: &str) -> Parsed<Token> {
    let actual = text.chars().next().ok_or("Tried to parse empty token")?;
    if expected == actual {
        Ok(Token::new(ttype, &text[..1]))
    } else {
        Err(format!("found: {}, expected: {}", actual, expected))
    }
}

fn parse_spaces<N>(text: &str) -> Parsed<Token> {
    let l = text.chars().take_while(|c| *c == ' ').count();
    if l == 0 {
        Err("empty space token".to_owned())
    } else {
        Ok(Token::new(TokenType::Space, &text[..l]))
    }
}

fn parse_comma<N>(text: &str) -> Parsed<Token> {
    parse_char::<N>(',', TokenType::Comma, text)
}

fn parse_semicolon<N>(text: &str) -> Parsed<Token> {
    parse_char::<N>(';', TokenType::Semicolon, text)
}

fn parse_newline<N>(text: &str) -> Parsed<Token> {
    parse_char::<N>('\n', TokenType::Newline, text)
}

fn parse_lbracket<N>(text: &str) -> Parsed<Token> {
    parse_char::<N>('[', TokenType::LBracket, text)
}

fn parse_rbracket<N>(text: &str) -> Parsed<Token> {
    parse_char::<N>(']', TokenType::RBracket, text)
}

fn parse_lparen<N>(text: &str) -> Parsed<Token> {
    parse_char::<N>('(', TokenType::LParen, text)
}

fn parse_rparen<N>(text: &str) -> Parsed<Token> {
    parse_char::<N>(')', TokenType::RParen, text)
}

fn parse_comment<N>(text: &str) -> Parsed<Token> {
    let l = text.chars().take_while(|c| *c != '\n').count();
    if l == 0 {
        Err("empty comment".to_owned())
    } else {
        Ok(Token::new(TokenType::Comment, &text[..l]))
    }
}

fn parse_number<N: FromStr>(text: &str) -> Parsed<Token>
where
    <N as FromStr>::Err: std::fmt::Debug,
{
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
    let parsed = lexeme.parse::<N>();
    if parsed.is_err() {
        return Err(format!("failed to parse '{}'", lexeme));
    }
    if l == 0 {
        Err("empty number".to_owned())
    } else {
        Ok(Token::lit(lexeme))
    }
}

fn parse_identifier(text: &str) -> Parsed<&str> {
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
    Ok(&text[..l])
}

fn parse_known_identifier<'a, N: Real + FromStr>(
    text: &'a str,
    env: &Env<N>,
) -> Parsed<Token<'a>> {
    let lexeme = parse_identifier(text)?;
    match env.find_unary_or_literal(lexeme) {
        Err(msg) => Err(msg),
        Ok(Expr::Literal(n)) => Ok(Token::new(TokenType::Literal, lexeme)),
        Ok(Expr::Function(f)) => Ok(Token::new(TokenType::Identifier, lexeme)),
        _ => unimplemented!(),
    }
}

fn parse_symbol<N>(text: &str) -> Parsed<Token> {
    let actual = text.chars().next().ok_or("there should be a char here")?;
    if SYMBOLS.contains(actual) {
        Ok(Token::sym(&text[..1]))
    } else {
        Err(format!("expected binary, found: {}", actual))
    }
}

#[cfg(test)]
mod test {
    use crate::env;

    use super::*;

    // #[test]
    // fn _assignment() {
    //     let input = "x = 6";
    //     let expected = Stmt::Assignment("x".to_owned(), vec![Token::lit(6., "6")]);
    //     assert_eq!(parse(input, &Env::<f64>::prelude()).unwrap(), expected);
    // }

    // #[test]
    // fn _z() {
    //     let input = "2 (+3+5)";
    //     let actual = parse(input, &env::Env::<f64>::prelude()).unwrap();
    //     let expected = Stmt::Expr(vec![
    //         Token::lit(2., "2"),
    //         Token::space(),
    //         Token::lparen(),
    //         Token::sym("+"),
    //         Token::lit(3., "3"),
    //         Token::sym("+"),
    //         Token::lit(5., "5"),
    //         Token::rparen(),
    //     ]);
    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn _spaces() {
        let spaces = "       ";
        assert_eq!(
            parse(spaces, &env::Env::<f64>::prelude()).unwrap(),
            Stmt::Expr(vec![Token::new(TokenType::Space, spaces)])
        );
    }

    // #[test]
    // fn _parse() {
    //     let expected = Ok(Stmt::Expr(vec![
    //         Token::lit(234., "234"),
    //         Token::sym("*"),
    //         Token::lit(5., "5"),
    //         Token::sym("+"),
    //         Token::lit(7., "7"),
    //         Token::sym("*"),
    //         Token::lit(8., "8"),
    //         Token::sym("-"),
    //         Token::lit(18., "18"),
    //         Token::sym("^"),
    //         Token::lit(3., "3"),
    //     ]));
    //     assert_eq!(parse("234*5+7*8-18^3", &env::Env::<f64>::prelude()), expected);
    // }

    // #[test]
    // fn _a() {
    //     let to_parse = "-(5+6)";
    //     let expected = Ok(Stmt::Expr(vec![
    //         Token::sym("-"),
    //         Token::lparen(),
    //         Token::lit(5., "5"),
    //         Token::sym("+"),
    //         Token::lit(6., "6"),
    //         Token::rparen(),
    //     ]));
    //     assert_eq!(parse(to_parse, &env::Env::<f64>::prelude()), expected);
    // }

    // #[test]
    // fn _b() {
    //     let to_parse = "-1 +4";
    //     let expected = Ok(Stmt::Expr(vec![
    //         Token::sym("-"),
    //         Token::lit(1., "1"),
    //         Token::space(),
    //         Token::sym("+"),
    //         Token::lit(4., "4"),
    //     ]));
    //     assert_eq!(parse(to_parse, &env::Env::<f64>::prelude()), expected);
    // }

    // //" -(6) * -(6)"
    // #[test]
    // fn _d() {
    //     let to_parse = " -(6) * -(6)";
    //     assert_eq!(
    //         parse(to_parse, &env::Env::<f64>::prelude()),
    //         Ok(Stmt::Expr(vec![
    //             Token::space(),
    //             Token::sym("-"),
    //             Token::lparen(),
    //             Token::lit(6., "6"),
    //             Token::rparen(),
    //             Token::space(),
    //             Token::sym("*"),
    //             Token::space(),
    //             Token::sym("-"),
    //             Token::lparen(),
    //             Token::lit(6., "6"),
    //             Token::rparen(),
    //         ]))
    //     );
    // }
}
