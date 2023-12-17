use crate::env::Env;
use crate::function::*;
use crate::interpreter::Expr;
use crate::parser::Parsed;
use crate::token::*;

pub const PRIORITY_SPACE: i32 = 10;
pub const PRIORITY_PAREN: i32 = 2 * PRIORITY_SPACE;

struct Exprs {
    stack: Vec<Option<Expr>>,
    balance: i32,
}

impl Exprs {
    fn new() -> Self {
        Self {
            stack: vec![],
            balance: 0,
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
        self.stack.push(Some(Expr::Function(
            f.clone()
                .prioritize(PRIORITY_SPACE * (self.balance - spaces)),
        )))
    }

    fn before_expr(&mut self) {
        let (spaces, last) = self.trim_trailing_spaces();

        if let Some(Some(Expr::Literal(_))) = last {
            self.imply(MUL, spaces);
        }
    }

    fn lparen(&mut self) {
        self.before_expr();
        self.balance += 2;
    }

    fn rparen(&mut self) -> Parsed<()> {
        self.balance -= 2;
        if self.balance < -1 {
            return Err("unmatched )".to_owned());
        }
        // account for " )"
        if self.balance % 2 == 1 {
            self.balance += 1;
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
                bin.prioritize(-PRIORITY_SPACE);
            }
            _ => {
                self.stack.push(None);
            }
        }
    }

    fn operator(&mut self, env: &Env, lexeme: &str) -> Parsed<()> {
        if let Expr::Function(binary) = env.find_binary(lexeme)? {
            let (spaces, last) = self.trim_trailing_spaces();
            let f = if let Ok(Expr::Function(unary)) = env.find_unary(lexeme) {
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
pub fn prioritize(tokens: Vec<Token>, env: &crate::env::Env) -> Parsed<Vec<Expr>> {
    // Inside `stack`, `None` indicates a space.
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
            TokenType::Identifier => match env.find_unary(tok.lexeme) {
                Ok(expr @ Expr::Function(_) | expr @ Expr::Literal(_)) => {
                    exprs.before_expr();
                    exprs.push(expr);
                }
                _ => exprs.push(Expr::Variable(tok.lexeme.to_owned())),
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
            prioritize(Vec::from(it), &Env::std()),
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
            prioritize(Vec::from(it), &Env::std()),
            Ok(vec![
                Expr::Literal(8.),
                Expr::Function(SUB.prioritize(-PRIORITY_SPACE)),
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
            prioritize(Vec::from(it), &Env::std()), // (5+ -6)-7
            Ok(vec![
                Expr::Literal(5.),
                Expr::Function(ADD.prioritize(PRIORITY_PAREN - PRIORITY_SPACE)),
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
            prioritize(Vec::from(it), &Env::std()),
            Ok(vec![
                Expr::Function(NEG),
                Expr::Literal(5.),
                Expr::Function(ADD.prioritize(PRIORITY_SPACE)),
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
                ],
                &Env::std()
            ),
            Ok(vec![
                Expr::Function(NEG),
                Expr::Literal(6.),
                Expr::Function(MUL.prioritize(-PRIORITY_SPACE)),
                Expr::Function(NEG),
                Expr::Literal(6.),
            ])
        );
    }
}
