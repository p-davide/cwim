use crate::env;
use crate::function::Function;
use crate::interpreter;
use crate::prioritize::Priority;
use crate::token::Token;
use crate::token::TokenType;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum S {
    Var(f64),
    Fun(String, Vec<S>),
}
impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Var(i) => write!(f, "{}", i),
            S::Fun(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
                }
                write!(f, ")")
            }
        }
    }
}
fn expr(lexer: &mut Vec<Token>, env: &env::Env) -> S {
    lexer.reverse();
    expr_bp(lexer, env, Priority::min())
}

fn expr_bp(lexer: &mut Vec<Token>, env: &env::Env, min_binding: Priority) -> S {
    let mut lhs = match lexer.pop() {
        Some(t) => match t.ttype {
            TokenType::Literal(n) => S::Var(n),
            TokenType::Symbol => {
                let ((), right) = prefix_binding_power(t.lexeme, env);
                let rhs = expr_bp(lexer, env, right);
                S::Fun(t.lexeme.to_owned(), vec![rhs])
            }
            TokenType::LParen => {
                let lhs = expr_bp(lexer, env, Priority::min());
                assert_eq!(lexer.pop().map(|it| it.ttype), Some(TokenType::RParen));
                lhs
            }
            _ => unreachable!("unexpected token {:?}", t.lexeme),
        },
        _ => unreachable!("empty lexer"),
    };
    loop {
        let op = match lexer.last().copied() {
            None => break,
            Some(t) => match t.ttype {
                TokenType::Symbol | TokenType::RParen => t.lexeme,
                t => unreachable!("{:?} with lexer state {:?}", t, lexer),
            },
        };
        if let Some((mut left, mut right)) = infix_binding_power(op, env) {
            if left < min_binding {
                break;
            }
            lexer.pop();
            let rhs = expr_bp(lexer, env, right);
            lhs = S::Fun(op.to_owned(), vec![lhs, rhs]);
            continue;
        }
        break;
    }
    lhs
}

fn infix_binding_power(op: &str, env: &env::Env) -> Option<(Priority, Priority)> {
    match env.find_binary_or_literal(op) {
        Ok(interpreter::Expr::Function(f)) => {
            let it = f.precedence.op_priority;
            Some((Priority::new(it * 2), Priority::new(it * 2 + 1)))
        }
        _ => None,
    }
}
fn prefix_binding_power(op: &str, env: &env::Env) -> ((), Priority) {
    match env.find_unary_or_literal(op) {
        Ok(interpreter::Expr::Function(f)) => {
            let it = f.precedence.op_priority;
            ((), Priority::new(it * 2 + 1))
        }
        _ => panic!("bad op: {:?}", op),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser;

    fn tokenize_and_parse(input: &str, expected: &str) {
        let mut tokens = parser::parse(input).unwrap();
        let actual = expr(&mut tokens, &mut env::Env::prelude());
        assert_eq!(actual.to_string(), expected);
    }
    #[test]
    fn _a() {
        tokenize_and_parse("1", "1");
    }
    #[test]
    fn _b() {
        tokenize_and_parse("1+2*3", "(+ 1 (* 2 3))");
    }
    #[test]
    fn _c() {
        tokenize_and_parse("--1", "(- -1)");
    }
    #[test]
    fn _d() {
        tokenize_and_parse("--1*2", "(* (- -1) 2)");
    }
    #[test]
    fn _e() {
        tokenize_and_parse("(((0)))", "0");
    }
    #[test]
    fn _f() {
        tokenize_and_parse("- -1*2", "(- (* -1) 2)");
    }
    #[test]
    fn _g() {
        tokenize_and_parse("1+2 * 3", "(* (+ 1 2) 3)");
    }
}