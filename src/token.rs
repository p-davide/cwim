#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Literal(f64),
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
    Error,
}

pub const ERROR: Token = Token {
    ttype: TokenType::Error,
    lexeme: ""
};

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token<'t> {
    pub ttype: TokenType,
    pub lexeme: &'t str,
}
