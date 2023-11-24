#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum TokenType {
    Literal,
    Identifier,
    Binary,
    Space,
    Comment,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Newline,
    Comma,
    Semicolon,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct Token<'t> {
    ttype: TokenType,
    lexeme: &'t str,
}

struct ParseState<'a> {
    to_parse: &'a str,
    tokens: Vec<Token<'a>>,
}

fn parse(text: &str) -> Option<Vec<Token>> {
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

// -- interpreter

#[derive(PartialEq)]
enum Expr {
    Literal(f64),
    Binary(Binary),
    Variable(String),
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Expr::Literal(n) => write!(f, "L{:?}", n),
            Expr::Binary(n) => write!(f, "B{:?}", n),
            Expr::Variable(n) => write!(f, "V{:?}", n),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Binary {
    name: &'static str,
    f: fn(f64,f64) -> f64,
    precedence: usize,
}

impl std::fmt::Debug for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{}:{}]", self.name, self.precedence)
    }
}

const add: Binary = Binary {
    name: "+",
    f: |x,y| x+y,
    precedence: 4,
};

const sub: Binary = Binary {
    name: "-",
    f: |x,y| x-y,
    precedence: 4,
};

const mul: Binary = Binary {
    name: "*",
    f: |x,y| x*y,
    precedence: 5,
};

const div: Binary = Binary {
    name: "/",
    f: |x,y| x/y,
    precedence: 5,
};

type InterpretError = ();

fn understand(tokens: Vec<Token>) -> Option<Vec<Expr>> {
    let mut result: Vec<Expr> = vec![];
    for tok in tokens {
        match understand_one(tok) {
            Some(x) => result.push(x),
            None => return None,
        }
    }
    Some(result)
}

fn understand_one(tok: Token) -> Option<Expr> {
    match tok.ttype {
        TokenType::Literal => match tok.lexeme.parse::<f64>() {
            Ok(n) => Some(Expr::Literal(n)),
            Err(_) => None,
        },
        TokenType::Identifier => Some(Expr::Variable(tok.lexeme.to_owned())),
        TokenType::Binary => match tok.lexeme {
            "+" => Some(Expr::Binary(add)),
            "-" => Some(Expr::Binary(sub)),
            "*" => Some(Expr::Binary(mul)),
            "/" => Some(Expr::Binary(div)),
            _ => None,
        },
        _ => unimplemented!(),
    }
}

// -- precedence

fn shuntingyard(exprs: Vec<Expr>) -> Option<Vec<Expr>> {
    let mut result = vec![];
    let mut ops: Vec<Binary> = vec![];
    for expr in exprs {
        match expr {
            Expr::Literal(_) | Expr::Variable(_) => result.push(expr),
            Expr::Binary(b) => {
                while let Some(op) = ops.last() {
                    if b.precedence < op.precedence {
                        result.push(Expr::Binary(ops.pop()?))
                    } else {
                        break
                    }
                }
                ops.push(b)
            }
        }
    }
    ops.reverse();
    for op in ops {
        result.push(Expr::Binary(op))
    }
    Some(result)
}

#[test]
fn _shuntingyard() {
    assert_eq!(shuntingyard(vec![
        Expr::Literal(234.0),
        Expr::Binary(mul),
        Expr::Literal(5.0),
    ]), Some(vec![
        Expr::Literal(234.0),
        Expr::Literal(5.0),
        Expr::Binary(mul),
    ]));
}
#[test]
fn _shuntingyard_2() {
    assert_eq!(shuntingyard(vec![
        Expr::Literal(234.0),
        Expr::Binary(mul),
        Expr::Literal(5.0),
        Expr::Binary(add),
        Expr::Literal(7.0),
        Expr::Binary(mul),
        Expr::Literal(8.0),
    ]), Some(vec![
        Expr::Literal(234.0),
        Expr::Literal(5.0),
        Expr::Binary(mul),
        Expr::Literal(7.0),
        Expr::Literal(8.0),
        Expr::Binary(mul),
        Expr::Binary(add),
    ]));
}

// -- eval

fn eval(shunted: Vec<Expr>) -> Option<f64> {
    let mut stack = vec![];
    for expr in shunted {
        match expr {
            Expr::Literal(n) => stack.push(n),
            Expr::Binary(b) => {
                let x = stack.pop()?;
                let y = stack.pop()?;
                let f = b.f;
                stack.push(f(x,y));
            },
            _ => unimplemented!(),
        }
    }
    stack.pop()
}

// -- main

fn run(text: &str) -> Option<f64> {
    let tks = parse(text)?;
    let exprs = understand(tks)?;
    let s = shuntingyard(exprs)?;
    if let Some(n) = eval(s) {
        Some(n)
    } else {
        None
    }
}

fn main() {
    //let expr1 = "(234 + 400) * 8";
    //let expr2 = "[234x + 400 1222] * [8; 10] # this is a comment";
    let expr3 = "234*5+7*8";
    println!("{:?}", run(expr3));
}
