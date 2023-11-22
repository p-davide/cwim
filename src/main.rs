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
struct Token {
    ttype: TokenType,
    length: usize,
}

struct ParseState<'a> {
    to_parse: &'a str,
    tokens: Vec<Token>,
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
                state.to_parse = &state.to_parse[token.length..]
            }
            Err(err) => return Err(err)
        }
    }
    Ok(state.tokens)
}

fn parse_token(text: &str) -> Result<Token, ParseError> {
    let it = Token {
        ttype: TokenType::Space,
        length: 1,
    };
    Ok(it)
}

fn main() {
    println!("{:?}", parse("lmao"));
}
