use crate::{env::Env, token::*};

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

pub fn parse<'a>(text: &'a str, env: &Env) -> Parsed<Stmt<'a>> {
    let mut tokens = vec![];

    let mut sides = text.split('=');
    let mut lhs = sides.next().expect("no =?");
    while !lhs.is_empty() {
        let token = parse_token(lhs, env)?;
        tokens.push(token);
        lhs = &lhs[token.lexeme.len()..];
    }
    if let Some(mut rhs) = sides.next() {
        let mut right_tokens = vec![];
        while !rhs.is_empty() {
            let token = parse_token(rhs, env)?;
            right_tokens.push(token);
            rhs = &rhs[token.lexeme.len()..];
        }
        Ok(Stmt::Assignment(tokens, right_tokens))
    } else {
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
    match env.get(lexeme) {
        Some(crate::env::Variable::Value(n)) => Ok(Token::new(TokenType::Literal(*n), lexeme)),
        _ => Ok(Token::new(TokenType::Identifier, lexeme)),
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
    fn _assignment() {
        assert_eq!(
            parse("x=6", &env::Env::prelude()).unwrap(),
            Stmt::Assignment(
                vec![Token::new(TokenType::Identifier, "x")],
                vec![Token::lit(6., "6")]
            )
        );
        assert_eq!(
            parse("7x+5y", &env::Env::prelude()).unwrap(),
            Stmt::Expr(vec![
                Token::lit(7., "7"),
                Token::new(TokenType::Identifier, "x"),
                Token::new(TokenType::Symbol, "+"),
                Token::lit(5., "5"),
                Token::new(TokenType::Identifier, "y"),
            ])
        )
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
