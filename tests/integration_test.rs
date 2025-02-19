use cwim::env::*;
use cwim::interpreter::run;
use cwim::number::Number;
use num::BigInt;
fn _test_run(text: &str, expected: Number) {
    assert_eq!(run(text, &mut Env::prelude()), Ok(expected));
}

fn _test_run_int(text: &str, expected: i64) {
    _test_run(text, Number::from(expected))
}

fn _test_run_float(text: &str, expected: f64) {
    _test_run(text, Number::Flt(expected))
}

#[test]
fn _run_basic() {
    _test_run_int("234*5+7*8-18^3", 234 * 5 + 7 * 8 - (18i64).pow(3));
}

#[test]
fn _run_with_spaces_1() {
    _test_run_int("234 * 5+7*8-18 ^ 3", 234 * (5 + 7 * 8 - 18i64).pow(3));
}

#[test]
fn _minus() {
    _test_run_int("3 - 2 - 1", 0);
    _test_run_int("3-2-1", 0);
}

#[test]
fn _plus_minus() {
    _test_run_int("5-+5", 0);
    _test_run_int("5+-5", 0);
}

#[test]
fn _run_with_spaces_2() {
    _test_run_int("234*5+7*8-18 ^ 3", (234 * 5 + 7 * 8 as i64 - 18).pow(3));
}

#[test]
fn _reg_1() {
    _test_run_float("2(log10 * 7)", 14.);
}

#[test]
fn _a() {
    _test_run_int("6+1*9", 6 + 1 * 9);
    _test_run_int("6 + 1 * 9", 6 + 1 * 9);
    _test_run_int("5 + 6 + 1 * 9", 5 + 6 + 1 * 9);
    _test_run_int("2 ^ 4 * 5 + 6 + 1 ^ 9", 2i64.pow(4) * 5 + 6 + 1i64.pow(9))
}

#[test]
fn _cos_2pi() {
    _test_run_float("cos 2pi", 1.)
}

#[test]
fn _run_with_spaces_3() {
    _test_run_int("234 *5+7*8-18 ^ 3", 234 * (5 + 7 * 8 - 18i64).pow(3));
}

#[test]
fn _run_with_spaces_4() {
    _test_run_int("234 * 5+7*8-18 ^ 3", 234 * (5 + 7 * 8 - 18 as i64).pow(3));
}

#[test]
fn _implied_multiplication() {
    _test_run_int("2(8)", 16);
    _test_run_int("(+3+5)", 8);
}

#[test]
fn _implied_multiplication_2() {
    _test_run_int("2(+3+5)", 16);
    _test_run_int("2 (+3+5)", 16);
}

#[test]
fn _implied_multiplication_3() {
    _test_run_int("2 3", 6);
}

#[test]
fn _run_with_parens_3() {
    _test_run_int("234 *(5+7*8-18) ^ 3", 234 * (5 + 7 * 8 - 18 as i64).pow(3));
}

#[test]
fn _just_a_number() {
    _test_run_int("3", 3);
    _test_run_int("(3)", 3);
    _test_run_int(" 3", 3);
    _test_run_int(" (3)", 3);
    _test_run_int(" 3 ", 3);
    _test_run_int(" (3 )", 3);
}

#[test]
fn _unmatched_parens() {
    assert_eq!(run("4)", &mut Env::prelude()), Ok(Number::from(4)));
}

#[test]
fn _negation() {
    assert_eq!(run("-4", &mut Env::prelude()), Ok(Number::from(-4)));
}

#[test]
fn _spaces_brackets_and_negations() {
    _test_run_int(" -(6) * -(6)", 36);
}

#[test]
fn _unary_ordering() {
    _test_run_float("cos2pi   ", (2. * std::f64::consts::PI).cos());
    _test_run_float("cos 2pi  ", (2. * std::f64::consts::PI).cos());
    _test_run_float("cos2 pi  ", 2f64.cos() * std::f64::consts::PI);
    _test_run_float("cos 2 pi ", (2. * std::f64::consts::PI).cos());
}

#[test]
fn _unary_and_binary() {
    _test_run_float("cos pi + 1", 0.);
}

#[test]
fn _double_unary() {
    _test_run_float("sin cos 2-2", (1 as f64).sin());
    _test_run_float("sin(cos 2-2)", (1 as f64).sin());
    _test_run_float("sin(cos(2-2))", (1 as f64).sin());
}

#[test]
fn _1st_deg_poly() {
    let mut env = Env::prelude();
    let _ = run("7x = 14", &mut env);
    assert_eq!(
        env.find_value("x"),
        Ok(cwim::interpreter::Expr::Literal(Number::from(2)))
    );
}

// TODO: Test assignments more.
#[test]
fn _assignment() {
    let mut env = Env::prelude();
    let _ = run("x = 6", &mut env);
    assert_eq!(
        env.find_value("x"),
        Ok(cwim::interpreter::Expr::Literal(Number::from(6)))
    );
    let _ = run("7z+5z = 12", &mut env);
    assert_eq!(
        env.find_value("z"),
        Ok(cwim::interpreter::Expr::Literal(Number::from(1)))
    );
}

#[test]
fn _assignment2() {
    let mut env = Env::prelude();
    let _ = run("(sin pi/2)y=1", &mut env);
    assert_eq!(
        env.find_value("y"),
        Ok(cwim::interpreter::Expr::Literal(Number::Flt(1.)))
    );
}

#[test]
fn _bignum() {
    assert_eq!(
        run("2^128", &mut Env::prelude()),
        Ok(Number::Int(
            BigInt::parse_bytes(b"340282366920938463463374607431768211456", 10).unwrap()
        ))
    );
    assert_eq!(
        run(
            "0xffffffffffffffffffffffffffffffffffffffffff",
            &mut Env::prelude()
        ),
        Ok(Number::Int(
            BigInt::parse_bytes(b"374144419156711147060143317175368453031918731001855", 10)
                .unwrap()
        ))
    );
}

#[test]
fn _divide_by_zero() {
    assert!(run("-1/0", &mut Env::prelude()).is_ok_and(|it| it.is_nan()));
    assert!(run("-1/-0", &mut Env::prelude()).is_ok_and(|it| it.is_nan()));
}

#[test]
fn _fractional_exponents() {
    _test_run_float("2^ -1/2", 0.7071067811865476);
}
