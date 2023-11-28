use crate::parser::{Parsed, SYMBOLS};
use crate::token::*;

struct ParseState<'a> {
    to_parse: &'a str,
    tokens: Vec<Token<'a>>,
}
impl<'a> ParseState<'a> {
    fn str(&mut self, expected: &str, ttype: TokenType) -> Parsed<()> {
        let n = expected.len();
        if self.to_parse.starts_with(expected) {
            self.tokens.push(Token {
                ttype: ttype,
                lexeme: &self.to_parse[..n],
            });
            self.to_parse = &self.to_parse[n..];
            Ok(())
        } else {
            Err("".to_owned())
        }
    }

    fn lp(&mut self) -> Parsed<()> {
        self.str("(", TokenType::LParen)
    }

    fn rp(&mut self) -> Parsed<()> {
        self.str(")", TokenType::RParen)
    }

    fn pat<F: Fn(char) -> bool>(&mut self, maybe_ttype: Option<TokenType>, pred: F) -> Parsed<()> {
        let trimmed = self.to_parse.trim_start_matches(pred);
        let n = self.to_parse.len() - trimmed.len();
        if trimmed == self.to_parse {
            Err("".to_owned())
        } else {
            if let Some(ttype) = maybe_ttype {
                self.tokens.push(Token {
                    ttype: ttype,
                    lexeme: &self.to_parse[..n],
                });
            }
            self.to_parse = trimmed;
            Ok(())
        }
    }

    // lp space? lhs rhs* rp
    #[allow(unused_must_use)]
    fn top(&mut self) -> Parsed<()> {
        self.lp()?;
        self.spaces();

        self.lhs()?;

        self.zero_plus_rhs()?;

        self.rp()
    }

    fn zero_plus_rhs(&mut self) -> Parsed<()> {
        while let Ok(_) = self.rhs() {
            continue;
        }
        Ok(())
    }

    fn spaces(&mut self) -> Parsed<()> {
        self.pat(Some(TokenType::Space), |c| c.is_ascii_whitespace())
    }

    // (space? bin space? side space?)
    #[allow(unused_must_use)]
    fn rhs(&mut self) -> Parsed<()> {
        self.spaces();
        self.pat(Some(TokenType::Binary), |c| SYMBOLS.contains(c))?;
        self.spaces();
        self.lhs()?;
        self.spaces()
    }

    // lit | top
    fn lhs(&mut self) -> Parsed<()> {
        self.lit().or_else(|_| self.top())
    }

    fn lit(&mut self) -> Parsed<()> {
        self.pat(Some(TokenType::Literal), |c: char| c.is_ascii_digit())
    }
}

pub fn parse(text: &str) -> Parsed<Vec<Token>> {
    let mut state = ParseState {
        to_parse: text,
        tokens: vec![],
    };
    state.top()?;
    Ok(state.tokens)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _parse_token() {
        assert_eq!(
            parse("(2 *2)"),
            Ok(vec![
                Token {
                    ttype: TokenType::LParen,
                    lexeme: "("
                },
                Token {
                    ttype: TokenType::Literal,
                    lexeme: "2"
                },
                Token {
                    ttype: TokenType::Space,
                    lexeme: " "
                },
                Token {
                    ttype: TokenType::Binary,
                    lexeme: "*"
                },
                Token {
                    ttype: TokenType::Literal,
                    lexeme: "2"
                },
                Token {
                    ttype: TokenType::RParen,
                    lexeme: ")"
                },
            ])
        )
    }
}
