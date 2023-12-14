use crate::token::*;

pub type Parsed<T> = Result<T, String>;

pub fn parse(text: &str) -> Parsed<Vec<Token>> {
    let mut to_parse = text;
    let mut tokens = vec![];
    while !to_parse.is_empty() {
        let token = parse_token(to_parse)?;
        if token.ttype == TokenType::Error {
            return Err(token.lexeme.to_owned());
        }
        tokens.push(token);
        to_parse = &to_parse[token.lexeme.len()..];
    }
    Ok(tokens)
}

fn parse_token(text: &str) -> Parsed<Token> {
    let c = text.chars().next().ok_or("Tried to parse empty token")?;
    if c.is_ascii_digit() {
        return parse_number(text);
    }
    if c.is_ascii_alphabetic() {
        return parse_identifier(text);
    }
    match c {
        '-' => parse_number(text).or_else(|_| parse_symbol(text)),
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
        Ok(Token::new(TokenType::Identifier, &text[..l]))
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
    use super::*;

    #[test]
    fn _parse() {
        let actual = parse("234*5+7*8-18^3").map(|ts| ts.iter().map(|t| t.lexeme).collect());
        let expected: Parsed<Vec<Token>> = Ok(vec![
            Token::lit(234., "234"),
            Token::sym("*"),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::lit(7., "7"),
            Token::sym("*"),
            Token::lit(8., "8"),
            Token::lit(-18., "-18"),
            Token::sym("^"),
            Token::lit(3., "3"),
        ]);
        assert_eq!(
            actual,
            Ok(vec!["234", "*", "5", "+", "7", "*", "8", "-18", "^", "3",])
        );
        assert_eq!(parse("234*5+7*8-18^3"), expected);
    }

    #[test]
    fn _a() {
        let to_parse = "-(5+6)";
        let actual = parse(to_parse).map(|ts| ts.iter().map(|t| t.lexeme).collect());
        let expected: Parsed<Vec<Token>> = Ok(vec![
            Token::sym("-"),
            Token::lparen(),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::lit(6., "6"),
            Token::rparen(),
        ]);
        assert_eq!(parse(to_parse), expected);
        assert_eq!(actual, Ok(vec!["-", "(", "5", "+", "6", ")"]));
    }

    #[test]
    fn _b() {
        let to_parse = "-1 +4";
        let actual = parse(to_parse).map(|ts| ts.iter().map(|t| t.lexeme).collect());
        let expected: Parsed<Vec<Token>> = Ok(vec![
            Token::lit(-1., "-1"),
            Token::space(),
            Token::sym("+"),
            Token::lit(4., "4"),
        ]);
        assert_eq!(parse(to_parse), expected);
        assert_eq!(actual, Ok(vec!["-1", " ", "+", "4"]));
    }

    #[test]
    fn _c() {
        let to_parse = "-( -1 +4)";
        let actual = parse(to_parse).map(|ts| ts.iter().map(|t| t.lexeme).collect());
        assert_eq!(actual, Ok(vec!["-", "(", " ", "-1", " ", "+", "4", ")"]));
    }
    //" -(6) * -(6)"
    #[test]
    fn _d() {
        let to_parse = " -(6) * -(6)";
        assert_eq!(
            parse(to_parse),
            Ok(vec![
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
            ])
        );
    }
}
