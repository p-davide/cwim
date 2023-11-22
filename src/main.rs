#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum TokenType {
    Literal,
    Identifier,
    Binary,
    Space,
    Comment,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Newline,
    Comma,
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

type ParseError = ();

fn parse(text: &str) -> Result<Vec<Token>, ParseError> {
    let mut state = ParseState {
        to_parse: text,
        tokens: vec![],
    };
    while state.to_parse.len() != 0 {
        match parse_token(state.to_parse) {
            Ok(token) => {
                state.tokens.push(token);
                state.to_parse = &state.to_parse[token.lexeme.len()..]
            }
            Err(err) => return Err(err)
        }
    }
    Ok(state.tokens)
}

fn parse_token(text: &str) -> Result<Token, ParseError> {
    for c in text.chars() {
        match c {
            '0'..='9' => return parse_number(text),
            _ => { break; }
        }
    }
    Err(())
}

fn parse_number(text: &str) -> Result<Token, ParseError> {
    let mut l: usize = 0;
    for c in text.chars() {
        if c.is_ascii_digit() {
            l += 1;
        } else { break; }
    }
    if l == 0 {
        Err(())
    } else {
        let token = Token {
            ttype: TokenType::Literal,
            lexeme: &text[..l]
        };
        Ok(token)
    }
}
fn main() {
    println!("{:?}", parse("234"));
}
