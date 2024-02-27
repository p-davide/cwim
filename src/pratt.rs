use crate::env;
use crate::function::Function;
use crate::function::MUL;

use crate::prioritize::Priority;
use crate::s::S;
use crate::token::Token;
use crate::token::TokenType;

// Modified from https://github.com/matklad/minipratt

pub fn expr<'a>(lexer: &mut Vec<Token<'a>>, env: &env::Env) -> S<'a> {
    lexer.reverse();
    pop_if_space(lexer);
    expr_bp(lexer, env, Priority::MIN)
}

fn pop_if_space<'a>(lexer: &mut Vec<Token<'a>>) -> Option<Token<'a>> {
    if lexer.last().is_some_and(|it| it.ttype == TokenType::Space) {
        lexer.pop()
    } else {
        None
    }
}

fn pop_spaced_infix(lexer: &mut Vec<Token>) {
    pop_if_space(lexer);
    match lexer.last().map(|it| it.ttype) {
        Some(TokenType::Symbol) => {
            lexer.pop();
            pop_if_space(lexer);
        }
        _ => {}
    }
}

fn spaced_infix<'a>(lexer: &mut Vec<Token<'a>>) -> (u16, Option<Token<'a>>) {
    let pre_space = pop_if_space(lexer);
    let pre_spaces = pre_space.map_or(0, |it| it.lexeme.len() as u16);
    let maybe_token = lexer.pop();
    let post_space = pop_if_space(lexer);
    let mut post_spaces = post_space.map_or(0, |it| it.lexeme.len() as u16);
    // Ignore ending spaces
    if lexer.is_empty() {
        post_spaces = 0;
    }
    if let Some(t) = post_space {
        lexer.push(t);
    }
    if let Some(t) = maybe_token {
        lexer.push(t);
    }
    if let Some(t) = pre_space {
        lexer.push(t);
    }
    (std::cmp::max(pre_spaces, post_spaces), maybe_token)
}

fn get_infix_by_name(name: &str, env: &env::Env) -> Function {
    env.find_binary(name)
        .expect(&format!("Binary {} not found", name))
}

fn get_prefix_by_name(name: &str, env: &env::Env) -> Function {
    env.find_unary(name)
        .expect(&format!("Unary {} not found", name))
}

fn rhs<'a>(lexer: &mut Vec<Token<'a>>, env: &env::Env, right: u16) -> S<'a> {
    let spaces = pop_if_space(lexer).map_or(0, |it| it.lexeme.len() as u16);
    expr_bp(
        lexer,
        env,
        Priority {
            spaces,
            op_priority: right,
        },
    )
}

fn expr_bp<'a>(lexer: &mut Vec<Token<'a>>, env: &env::Env, min_priority: Priority) -> S<'a> {
    let mut lhs = match lexer.pop() {
        Some(t) => match t.ttype {
            TokenType::Literal(n) => S::Var(n),
            TokenType::Symbol => {
                let right = prefix_op_priority(t.lexeme, env)
                    .expect(&format!("unknown prefix operator {}", t.lexeme));
                let rhs = rhs(lexer, env, right);
                S::Fun(get_prefix_by_name(t.lexeme, env), vec![rhs])
            }
            TokenType::LParen => {
                pop_if_space(lexer);
                let lhs = expr_bp(lexer, env, Priority::MIN);
                // eof is assumed to close every (, such that eg -(5-6 = 1
                pop_if_space(lexer);
                assert_eq!(
                    lexer.pop().map_or(TokenType::RParen, |it| it.ttype),
                    TokenType::RParen
                );
                lhs
            }
            // TODO: This assumes every name is of a unary prefix function.
            //       Functions with more args should be supported.
            TokenType::Identifier => match env.get(t.lexeme) {
                Some(env::Variable::Function(fs)) => {
                    let rhs = match fs.unary.map(|it| it.priority * 2 + 1) {
                        Some(right) => rhs(lexer, env, right),
                        None => panic!("expected function"),
                    };
                    S::Fun(get_prefix_by_name(t.lexeme, env), vec![rhs])
                }
                Some(env::Variable::Value(n)) => S::Var(*n),
                _ => S::Unknown(t.lexeme),
            },
            _ => unreachable!("unexpected token {:?}, lexer state: {:?}", t.lexeme, lexer),
        },
        None => unreachable!("empty lexer"),
    };

    loop {
        let (spaces, maybe_token) = spaced_infix(lexer);
        let (spaces, op) = match maybe_token {
            None => break,
            Some(t) => {
                match t.ttype {
                    TokenType::Symbol | TokenType::RParen => (spaces, t.lexeme),
                    // Finding a ( here instead of an operator means the expression is like ...2(3+...
                    // We treat this as a multiplication.
                    TokenType::LParen => (0xffff, "*"),
                    TokenType::Literal(_) => (spaces, "*"),
                    TokenType::Identifier => match env.get(t.lexeme) {
                        Some(env::Variable::Function(fs)) => {
                            let rhs = match fs.unary.map(|it| it.priority * 2 + 1) {
                                Some(right) => rhs(lexer, env, right),
                                None => panic!("expected function"),
                            };
                            return S::Fun(MUL, vec![lhs, rhs]);
                        }
                        Some(env::Variable::Value(n)) => return S::Var(*n),
                        None => (spaces, "*"),
                    },
                    t => unreachable!("{:?} with lexer state {:?}", t, lexer),
                }
            }
        };

        if let Some((left, right)) = infix_op_priority(op, env) {
            let op_priority = Priority {
                spaces,
                op_priority: left,
            };
            if op_priority < min_priority {
                break;
            }
            pop_spaced_infix(lexer);
            let rhs = expr_bp(
                lexer,
                env,
                Priority {
                    spaces: std::cmp::min(min_priority.spaces, spaces),
                    op_priority: right,
                },
            );
            lhs = S::Fun(get_infix_by_name(op, env), vec![lhs, rhs]);
            continue;
        }
        break;
    }
    lhs
}

fn infix_op_priority(op: &str, env: &env::Env) -> Option<(u16, u16)> {
    match env.find_binary(op) {
        Ok(Function { priority, .. }) => Some((priority * 2, priority * 2 + 1)),
        _ => None,
    }
}

fn prefix_op_priority(op: &str, env: &env::Env) -> Option<u16> {
    match env.find_unary(op) {
        Ok(Function { priority, .. }) => Some(priority * 2 + 1),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::{self, Stmt};

    fn tokenize_and_parse(input: &str, expected: &str) {
        let stmt = parser::parse(input, &env::Env::prelude()).unwrap();
        match stmt {
            Stmt::Expr(mut tokens) => {
                let actual = expr(&mut tokens, &mut env::Env::prelude());
                assert_eq!(actual.to_string(), expected);
            }
            _ => panic!("expected expression"),
        }
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
        tokenize_and_parse("234*5", "(* 234 5)");
        tokenize_and_parse("234*5+7*8", "(+ (* 234 5) (* 7 8))");
        tokenize_and_parse("-6 *-6", "(* (- 6) (- 6))");
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
    #[test]
    fn _implied_multiplication_2() {
        tokenize_and_parse("2(+3+5)", "(* 2 (+ (+ 3) 5))");
        tokenize_and_parse("2 (+3+5)", "(* 2 (+ (+ 3) 5))");
    }
    #[test]
    fn _cos_2pi() {
        tokenize_and_parse("cos 2pi", "(cos (* 2 3.141592653589793))")
    }
    #[test]
    fn _unary_ordering() {
        tokenize_and_parse("cos2pi   ", "(cos (* 2 3.141592653589793))");
        tokenize_and_parse("cos 2pi  ", "(cos (* 2 3.141592653589793))");
        tokenize_and_parse("cos2 pi  ", "(* (cos 2) 3.141592653589793)");
        tokenize_and_parse("cos 2 pi ", "(cos (* 2 3.141592653589793))");
    }
    #[test]
    fn _n() {
        tokenize_and_parse("4 log100", "(* 4 (log 100))")
    }

    #[test]
    fn _o() {
        tokenize_and_parse(" sin0 x", "(* (sin 0) x)");
        tokenize_and_parse("7x", "(* 7 x)");
        tokenize_and_parse("7x+5y", "(+ (* 7 x) (* 5 y))");
        tokenize_and_parse("(sin1)x", "(* (sin 1) x)");
    }
}
