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
    pub column: usize,
}

impl<'t> Token<'t> {
    pub fn new(ttype: TokenType, lexeme: &'t str, column: usize) -> Self {
        Self {
            ttype,
            lexeme,
            column,
        }
    }

    pub fn lit(lit: f64, lexeme: &'t str, column: usize) -> Self {
        Self::new(TokenType::Literal(lit), lexeme, column)
    }

    pub fn sym(lexeme: &'t str, column: usize) -> Self {
        Self::new(TokenType::Symbol, lexeme, column)
    }

    #[cfg(test)]
    pub fn space(offset: usize) -> Self {
        Self::new(TokenType::Space, " ", offset)
    }

    #[cfg(test)]
    pub fn lparen(offset: usize) -> Self {
        Self::new(TokenType::LParen, "(", offset)
    }

    #[cfg(test)]
    pub fn rparen(offset: usize) -> Self {
        Self::new(TokenType::RParen, ")", offset)
    }
}
