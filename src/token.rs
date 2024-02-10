#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Literal(f64),
    Identifier,
    Symbol,
    Space,
    Comment,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Newline,
    Comma,
    Semicolon,
    Eof,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Token<'t> {
    pub ttype: TokenType,
    pub lexeme: &'t str,
}

impl<'t> Token<'t> {
    pub fn new(ttype: TokenType, lexeme: &'t str) -> Self {
        Self { ttype, lexeme }
    }

    pub fn lit(lit: f64, lexeme: &'t str) -> Self {
        Self::new(TokenType::Literal(lit), lexeme)
    }

    pub fn sym(lexeme: &'t str) -> Self {
        Self::new(TokenType::Symbol, lexeme)
    }

    #[cfg(test)]
    pub fn space() -> Self {
        Self::new(TokenType::Space, " ")
    }

    #[cfg(test)]
    pub fn lparen() -> Self {
        Self::new(TokenType::LParen, "(")
    }

    #[cfg(test)]
    pub fn rparen() -> Self {
        Self::new(TokenType::RParen, ")")
    }
}
