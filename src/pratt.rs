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
    Fun(Function, Vec<S>),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Var(i) => write!(f, "{}", i),
            S::Fun(head, rest) => {
                write!(f, "({}", head.name)?;
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

fn pop_trailing_space<'a>(lexer: &mut Vec<Token<'a>>) -> Option<Token<'a>> {
    if lexer.last().is_some_and(|it| it.ttype == TokenType::Space) {
        lexer.pop()
    } else {
        None
    }
}

fn infix_adjust_spaces<'a>(lexer: &mut Vec<Token<'a>>) -> (u16, Option<Token<'a>>) {
    let pre_spaces = pop_trailing_space(lexer).map_or(0, |it| it.lexeme.len() as u16);
    let token = lexer.pop();
    let post_spaces = pop_trailing_space(lexer).map_or(0, |it| it.lexeme.len() as u16);
    token.and_then(|t| Some(lexer.push(t)));
    (std::cmp::max(pre_spaces, post_spaces), token)
}

fn get_infix_by_name(name: &str, env: &env::Env) -> Function {
    match env.find_binary_or_literal(name) {
        Ok(interpreter::Expr::Function(f)) => f,
        _ => panic!("Unary {} not found", name),
    }
}

fn expr_bp(lexer: &mut Vec<Token>, env: &env::Env, min_binding: Priority) -> S {
    let mut lhs = match lexer.pop() {
        Some(t) => match t.ttype {
            TokenType::Literal(n) => S::Var(n),
            TokenType::Symbol => {
                let ((), right) = prefix_binding_power(t.lexeme, env);
                let spaces = pop_trailing_space(lexer).map_or(0, |it| it.lexeme.len() as u16);
                let rhs = expr_bp(lexer, env, Priority { spaces, ..right });
                S::Fun(get_infix_by_name(t.lexeme, env), vec![rhs])
            }
            TokenType::LParen => {
                pop_trailing_space(lexer);
                let lhs = expr_bp(lexer, env, Priority::min());
                // eof is assumed to close every (, such that eg -(5-6 = 1
                assert_eq!(
                    lexer.pop().map_or(TokenType::RParen, |it| it.ttype),
                    TokenType::RParen
                );
                lhs
            }
            _ => unreachable!("unexpected token {:?}, lexer state: {:?}", t.lexeme, lexer),
        },
        _ => unreachable!("empty lexer"),
    };

    loop {
        let (spaces, op) = match lexer.last().copied() {
            None => break,
            Some(t) => match t.ttype {
                TokenType::Symbol | TokenType::RParen | TokenType::Space => {
                    let (spaces, maybe_op) = infix_adjust_spaces(lexer);
                    let op = maybe_op.map_or("", |it| it.lexeme);
                    (spaces, op)
                }
                t => unreachable!("{:?} with lexer state {:?}", t, lexer),
            },
        };
        if let Some((left, right)) = infix_binding_power(op, env) {
            if (Priority { spaces, ..left }) < min_binding {
                break;
            }
            lexer.pop();
            let rhs = expr_bp(
                lexer,
                env,
                Priority {
                    spaces: std::cmp::min(min_binding.spaces, spaces),
                    ..right
                },
            );
            lhs = S::Fun(get_infix_by_name(op, env), vec![lhs, rhs]);
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

fn eval(s: &S) -> f64 {
    match s {
        S::Var(n) => *n,
        S::Fun(f, xs) => (f.f)(xs.iter().map(|ss| eval(ss)).collect()),
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
        tokenize_and_parse("--1", "(- (- 1))");
    }
    #[test]
    fn _d() {
        tokenize_and_parse("--1*2", "(* (- (- 1)) 2)");
    }
    #[test]
    fn _e() {
        tokenize_and_parse("(((0)))", "0");
    }
    #[test]
    fn _f() {
        tokenize_and_parse("- -1*2", "(- (* (- 1) 2))");
    }
    #[test]
    fn _g() {
        tokenize_and_parse("1+2 * 3", "(* (+ 1 2) 3)");
    }
    #[test]
    fn _h() {
        tokenize_and_parse("(((0", "0");
    }
    #[test]
    fn _i() {
        tokenize_and_parse("8.-9.", "(- 8 9)")
    }
    #[test]
    fn _j() {
        tokenize_and_parse("8. - 9.", "(- 8 9)")
    }
    #[test]
    fn _k() {
        tokenize_and_parse("(5+ -6)-7", "(- (+ 5 (- 6)) 7)")
    }
    #[test]
    fn _l() {
        tokenize_and_parse("-(5+ -6)-7", "(- (- (+ 5 (- 6))) 7)")
    }
    #[test]
    fn _m() {
        tokenize_and_parse("-(6) * -(6)", "(* (- 6) (- 6))")
    }
}
