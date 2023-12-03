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

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token<'t> {
    pub ttype: TokenType,
    pub lexeme: &'t str,
}

impl<'t> Token<'t> {
    pub fn new(ttype: TokenType, lexeme: &'t str) -> Self {
        Self {
            ttype: ttype,
            lexeme: lexeme,
        }
    }

    pub fn lit(lit: f64, lexeme: &'t str) -> Self {
        Self::new(TokenType::Literal(lit), lexeme)
    }

    pub fn bin(lexeme: &'t str) -> Self {
        Self::new(TokenType::Binary, lexeme)
    }

    #[cfg(test)]
    pub fn space() -> Token<'static> {
        Token::new(TokenType::Space, " ")
    }

    #[cfg(test)]
    pub fn lparen() -> Token<'static> {
        Token::new(TokenType::LParen, "(")
    }

    #[cfg(test)]
    pub fn rparen() -> Token<'static> {
        Token::new(TokenType::RParen, ")")
    }
}
