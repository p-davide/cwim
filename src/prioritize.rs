use crate::env::Env;
use crate::function::*;
use crate::interpreter::Expr;
use crate::parser::Parsed;
use crate::token::*;
use std::cmp::Ordering;
use std::fmt::Formatter;

#[derive(PartialEq, Eq, Ord, Clone, Copy)]
pub struct Priority {
    pub op_priority: u16,
    pub spaces: u16,
    pub parens: u16,
}
impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.parens.partial_cmp(&other.parens) {
            ord @ (Some(Ordering::Less) | Some(Ordering::Greater)) => ord,
            _ => match other.spaces.partial_cmp(&self.spaces) {
                ord @ (Some(Ordering::Less) | Some(Ordering::Greater)) => ord,
                _ => self.op_priority.partial_cmp(&other.op_priority),
            },
        }
    }
}
impl std::fmt::Debug for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}{}{}", self.parens, 9 - self.spaces, self.op_priority)
    }
}
impl Priority {
    pub const fn new(op_priority: u16) -> Self {
        Self {
            op_priority,
            spaces: 0,
            parens: 0,
        }
    }

    pub const fn min() -> Self {
        Self {
            op_priority: 0,
            spaces: 0xffff,
            parens: 0,
        }
    }
    
    pub fn paren(&mut self) -> Self {
        self.parens += 1;
        *self
    }
    pub fn space(&mut self) -> Self {
        self.spaces += 1;
        *self
    }
}

struct Exprs {
    // Inside `stack`, `None` indicates a space.
    stack: Vec<Option<Expr>>,
    open_parens: i32,
}

impl Exprs {
    fn new() -> Self {
        Self {
            stack: vec![],
            open_parens: 0,
        }
    }

    fn trim_trailing_spaces(&mut self) -> (i32, Option<&Option<Expr>>) {
        if let Some(None) = self.stack.last() {
            self.stack.pop().unwrap();
            (1, self.stack.last())
        } else {
            (0, self.stack.last())
        }
    }

    fn imply(&mut self, f: Function, spaces: i32) {
        let mut g = f.clone();
        g.precedence.parens = self.open_parens as u16;
        g.precedence.spaces = spaces as u16;
        self.stack.push(Some(Expr::Function(g)))
    }

    fn before_expr(&mut self) {
        let (spaces, last) = self.trim_trailing_spaces();

        if let Some(Some(Expr::Literal(_))) = last {
            self.imply(MUL, spaces);
        }
    }

    fn lparen(&mut self) {
        self.before_expr();
        self.open_parens += 2;
    }

    fn rparen(&mut self) -> Parsed<()> {
        self.open_parens -= 2;
        if self.open_parens < -1 {
            return Err("unmatched )".to_owned());
        }
        // account for " )"
        if self.open_parens % 2 == 1 {
            self.open_parens += 1;
        }
        Ok(())
    }

    fn lit(&mut self, n: f64) -> Parsed<()> {
        let (spaces, last) = self.trim_trailing_spaces();

        if let Some(Some(Expr::Literal(m))) = last {
            if spaces == 0 || n < 0. {
                self.imply(ADD, spaces);
            } else {
                return Err(format!("No operation specified for {} and {}", m, n));
            }
        }
        self.stack.push(Some(Expr::Literal(n)));
        Ok(())
    }

    fn push(&mut self, var: Expr) {
        self.stack.push(Some(var))
    }

    fn space(&mut self) {
        match self.stack.last_mut() {
            Some(Some(Expr::Function(bin))) if bin.arity <= 2 && !bin.was_spaced() => {
                bin.space();
            }
            _ => {
                self.stack.push(None);
            }
        }
    }

    fn operator(&mut self, env: &Env, lexeme: &str) -> Parsed<()> {
        if let Expr::Function(binary) = env.find_binary_or_literal(lexeme)? {
            let (spaces, last) = self.trim_trailing_spaces();
            let f = if let Ok(Expr::Function(unary)) = env.find_unary_or_literal(lexeme) {
                match last {
                    None | Some(Some(Expr::Function(_))) => unary,
                    _ => binary,
                }
            } else {
                binary
            };
            self.imply(f, spaces);
            Ok(())
        } else {
            Err(format!("no function named '{}'", lexeme))
        }
    }
}
pub fn prioritize<'a, Tokens: Iterator<Item = &'a Token<'a>>>(
    tokens: Tokens,
    env: &Env,
) -> Parsed<Vec<Expr>> {
    let mut exprs = Exprs::new();

    for tok in tokens {
        match tok.ttype {
            TokenType::LParen => {
                exprs.lparen();
            }
            TokenType::RParen => {
                exprs.rparen()?;
            }
            TokenType::Literal(n) => {
                exprs.lit(n)?;
            }
            TokenType::Identifier => match env.find_unary_or_literal(tok.lexeme) {
                Ok(expr @ Expr::Function(_) | expr @ Expr::Literal(_)) => {
                    exprs.before_expr();
                    exprs.push(expr);
                }
                _ => exprs.push(Expr::Variable(tok.lexeme.to_owned(), 1.)),
            },
            TokenType::Symbol => {
                exprs.operator(env, tok.lexeme)?;
            }
            TokenType::Space => {
                exprs.space();
            }
            TokenType::Comment => {
                break;
            }
            tt => unimplemented!("{:?}", tt),
        }
    }
    // Remove any leftover `None`s (spaces)
    Ok(exprs.stack.into_iter().flatten().collect())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn _simple_priority() {
        let it = [Token::lit(8., "8."), Token::sym("-"), Token::lit(9., "9")];
        assert_eq!(
            prioritize(it.iter(), &Env::prelude()),
            Ok(vec![
                Expr::Literal(8.),
                Expr::Function(SUB),
                Expr::Literal(9.),
            ])
        );
    }

    #[test]
    #[allow(const_item_mutation)]
    fn _with_spaces() {
        let it = [
            Token::lit(8., "8."),
            Token::space(),
            Token::sym("-"),
            Token::space(),
            Token::lit(9., "9"),
        ];
        assert_eq!(
            prioritize(it.iter(), &Env::prelude()),
            Ok(vec![
                Expr::Literal(8.),
                Expr::Function(SUB.space()),
                Expr::Literal(9.),
            ])
        );
    }

    // (5+ -6)-7
    #[test]
    #[allow(const_item_mutation)]
    fn _parens_and_spaces() {
        let it = [
            Token::lparen(),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::space(),
            Token::lit(-6., "-6"),
            Token::rparen(),
            Token::lit(-7., "-7"),
        ];
        assert_eq!(
            prioritize(it.iter(), &Env::prelude()), // (5+ -6)-7
            Ok(vec![
                Expr::Literal(5.),
                Expr::Function(ADD.paren().space()),
                Expr::Literal(-6.),
                Expr::Function(ADD),
                Expr::Literal(-7.),
            ])
        )
    }

    // -(5+ -6)-7
    #[test]
    #[allow(const_item_mutation)]
    fn _neg_parens_and_spaces() {
        let it = [
            Token::sym("-"),
            Token::lparen(),
            Token::lit(5., "5"),
            Token::sym("+"),
            Token::space(),
            Token::lit(-6., "-6"),
            Token::rparen(),
            Token::lit(-7., "-7"),
        ];
        assert_eq!(
            prioritize(it.iter(), &Env::prelude()),
            Ok(vec![
                Expr::Function(NEG),
                Expr::Literal(5.),
                Expr::Function(ADD.space()),
                Expr::Literal(-6.),
                Expr::Function(ADD),
                Expr::Literal(-7.),
            ])
        )
    }

    // " -(6) * -(6)"
    #[test]
    #[allow(const_item_mutation)]
    fn _nsix() {
        assert_eq!(
            prioritize(
                vec![
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
                ]
                .iter(),
                &Env::prelude()
            ),
            Ok(vec![
                Expr::Function(NEG),
                Expr::Literal(6.),
                Expr::Function(MUL.space()),
                Expr::Function(NEG),
                Expr::Literal(6.),
            ])
        );
    }
}
