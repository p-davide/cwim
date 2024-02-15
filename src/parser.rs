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

fn parse_assignee(text: &str) -> Parsed<(usize, String)> {
    let mut i = 0;

    if let Ok(token) = parse_spaces(text) {
        i += token.lexeme.len();
    }

    let assignee = parse_identifier(&text[i..])?;
    i += assignee.len();

    if let Ok(token) = parse_spaces(&text[i..]) {
        i += token.lexeme.len();
    }

    parse_char('=', TokenType::Space, &text[i..])?;
    i += 1;

    if let Ok(token) = parse_spaces(&text[i..]) {
        i += token.lexeme.len();
    }

    Ok((i, assignee.to_owned()))
}

pub fn parse<'a>(text: &'a str, env: &Env) -> Parsed<Stmt<'a>> {
    let mut tokens = vec![];
    if let Ok((n, assignee)) = parse_assignee(text) {
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

fn parse_token<'a>(text: &'a str, env: &Env) -> Parsed<Token<'a>> {
    let c = text.chars().next().ok_or("Tried to parse empty token")?;
    if c.is_ascii_digit() {
        return parse_number(text);
    }
    if c.is_ascii_alphabetic() {
        return parse_known_identifier(text, env);
    }
    match c {
        '-' => parse_symbol(text),
        ' ' => parse_spaces(text),
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
                parse_symbol(text)
            } else {
                Err(format!("Can't parse '{}'", c))
            }
        }
    }
}

pub const SYMBOLS: &str = "!@$%^&*|\"';,./+-=";

fn parse_char(expected: char, ttype: TokenType, text: &str) -> Parsed<Token> {
    let actual = text.chars().next().ok_or("Tried to parse empty token")?;
    if expected == actual {
        Ok(Token::new(ttype, &text[..1]))
    } else {
        Err(format!("found: {}, expected: {}", actual, expected))
    }
}

fn parse_spaces(text: &str) -> Parsed<Token> {
    let l = text.chars().take_while(|c| *c == ' ').count();
    if l == 0 {
        Err("empty space token".to_owned())
    } else {
        Ok(Token::new(TokenType::Space, &text[..l]))
    }
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
    let l = text.chars().take_while(|c| *c != '\n').count();
    if l == 0 {
        Err("empty comment".to_owned())
    } else {
        Ok(Token::new(TokenType::Comment, &text[..l]))
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
    let parsed = lexeme.parse::<f64>();
    if parsed.is_err() {
        return Err(format!("failed to parse '{}'", lexeme));
    }
    if l == 0 {
        Err("empty number".to_owned())
    } else {
        Ok(Token::lit(parsed.unwrap(), lexeme))
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

fn parse_known_identifier<'a>(text: &'a str, env: &Env) -> Parsed<Token<'a>> {
    let lexeme = parse_identifier(text)?;
    match env.find_unary_or_literal(lexeme) {
        Err(msg) => Err(msg),
        Ok(Expr::Literal(n)) => Ok(Token::new(TokenType::Literal(n), lexeme)),
        Ok(Expr::Function(f)) => Ok(Token::new(TokenType::Identifier, lexeme)),
        _ => unimplemented!(),
    }
}

fn parse_symbol(text: &str) -> Parsed<Token> {
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

    #[test]
    fn _assignment() {
        let input = "x = 6";
        let expected = Stmt::Assignment("x".to_owned(), vec![Token::lit(6., "6")]);
        assert_eq!(parse(input, &Env::prelude()).unwrap(), expected);
    }

    #[test]
    fn _z() {
        let input = "2 (+3+5)";
        let actual = parse(input, &env::Env::prelude()).unwrap();
        let expected = Stmt::Expr(vec![
            Token::lit(2., "2"),
            Token::space(),
            Token::lparen(),
            Token::sym("+"),
            Token::lit(3., "3"),
            Token::sym("+"),
            Token::lit(5., "5"),
            Token::rparen(),
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn _spaces() {
        let spaces = "       ";
        assert_eq!(
            parse(spaces, &env::Env::prelude()).unwrap(),
            Stmt::Expr(vec![Token::new(TokenType::Space, spaces)])
        );
    }

    #[test]
    fn _parse() {
        let expected = Ok(Stmt::Expr(vec![
            Token::lit(234., "234"),
            Token::sym("*"),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::lit(7., "7"),
            Token::sym("*"),
            Token::lit(8., "8"),
            Token::sym("-"),
            Token::lit(18., "18"),
            Token::sym("^"),
            Token::lit(3., "3"),
        ]));
        assert_eq!(parse("234*5+7*8-18^3", &env::Env::prelude()), expected);
    }

    #[test]
    fn _a() {
        let to_parse = "-(5+6)";
        let expected = Ok(Stmt::Expr(vec![
            Token::sym("-"),
            Token::lparen(),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::lit(6., "6"),
            Token::rparen(),
        ]));
        assert_eq!(parse(to_parse, &env::Env::prelude()), expected);
    }

    #[test]
    fn _b() {
        let to_parse = "-1 +4";
        let expected = Ok(Stmt::Expr(vec![
            Token::sym("-"),
            Token::lit(1., "1"),
            Token::space(),
            Token::sym("+"),
            Token::lit(4., "4"),
        ]));
        assert_eq!(parse(to_parse, &env::Env::prelude()), expected);
    }

    //" -(6) * -(6)"
    #[test]
    fn _d() {
        let to_parse = " -(6) * -(6)";
        assert_eq!(
            parse(to_parse, &env::Env::prelude()),
            Ok(Stmt::Expr(vec![
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
            ]))
        );
    }
}
