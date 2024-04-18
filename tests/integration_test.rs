use cwim::env::*;
use cwim::interpreter::run;
use cwim::number::Number;
fn _test_run(text: &str, expected: Number) {
    assert_eq!(run(text, &mut Env::prelude()), Ok(expected));
}

fn _test_run_scalar(text: &str, expected: f64) {
    _test_run(text, Number::scalar(expected))
}

#[test]
fn _run_basic() {
    _test_run_scalar("234*5+7*8-18^3", 234. * 5. + 7. * 8. - (18 as f64).powf(3.));
}

#[test]
fn _run_with_spaces_1() {
    _test_run_scalar(
        "234 * 5+7*8-18 ^ 3",
        234. * (5. + 7. * 8 as f64 - 18.).powf(3.),
    );
}

#[test]
fn _minus() {
    _test_run_scalar("3 - 2 - 1", 0.);
    _test_run_scalar("3-2-1", 0.);
}

#[test]
fn _plus_minus() {
    _test_run_scalar("5-+5", 0.);
    _test_run_scalar("5+-5", 0.);
}

#[test]
fn _run_with_spaces_2() {
    _test_run_scalar(
        "234*5+7*8-18 ^ 3",
        (234. * 5. + 7. * 8 as f64 - 18.).powf(3.),
    );
}

#[test]
fn _reg_1() {
    _test_run_scalar("2(log10 * 7)", 14.);
}

#[test]
fn _a() {
    _test_run_scalar("6+1*9", 6. + 1. * 9.);
    _test_run_scalar("6 + 1 * 9", 6. + 1. * 9.);
    _test_run_scalar("5 + 6 + 1 * 9", 5. + 6. + 1. * 9.);
    _test_run_scalar(
        "2 ^ 4 * 5 + 6 + 1 ^ 9",
        2f64.powf(4.) * 5. + 6. + 1f64.powf(9.),
    )
}

#[test]
fn _cos_2pi() {
    _test_run_scalar("cos 2pi", 1.)
}

#[test]
fn _run_with_spaces_3() {
    _test_run_scalar("234 *5+7*8-18 ^ 3", 234. * (5. + 7. * 8. - 18f64).powf(3.));
}

#[test]
fn _run_with_spaces_4() {
    _test_run_scalar(
        "234 * 5+7*8-18 ^ 3",
        234. * (5. + 7. * 8. - 18 as f64).powf(3.),
    );
}

#[test]
fn _implied_multiplication() {
    _test_run_scalar("2(8)", 16.);
    _test_run_scalar("(+3+5)", 8.);
}

#[test]
fn _implied_multiplication_2() {
    _test_run_scalar("2(+3+5)", 16.);
    _test_run_scalar("2 (+3+5)", 16.);
}

#[test]
fn _implied_multiplication_3() {
    _test_run_scalar("2 3", 6.);
}

#[test]
fn _run_with_parens_3() {
    _test_run_scalar(
        "234 *(5+7*8-18) ^ 3",
        234. * (5. + 7. * 8. - 18 as f64).powf(3.),
    );
}

#[test]
fn _just_a_number() {
    _test_run_scalar("3", 3.);
    _test_run_scalar("(3)", 3.);
    _test_run_scalar(" 3", 3.);
    _test_run_scalar(" (3)", 3.);
    _test_run_scalar(" 3 ", 3.);
    _test_run_scalar(" (3 )", 3.);
}

#[test]
fn _unmatched_parens() {
    assert_eq!(run("4)", &mut Env::prelude()), Ok(Number::scalar(4.)));
}

#[test]
fn _spaces_brackets_and_negations() {
    _test_run_scalar(" -(6) * -(6)", 36.);
}

#[test]
fn _unary_ordering() {
    _test_run_scalar("cos2pi   ", (2. * std::f64::consts::PI).cos());
    _test_run_scalar("cos 2pi  ", (2. * std::f64::consts::PI).cos());
    _test_run_scalar("cos2 pi  ", 2f64.cos() * std::f64::consts::PI);
    _test_run_scalar("cos 2 pi ", (2. * std::f64::consts::PI).cos());
}

#[test]
fn _double_unary() {
    _test_run_scalar("sin cos 2-2", (1 as f64).sin());
    _test_run_scalar("sin(cos 2-2)", (1 as f64).sin());
    _test_run_scalar("sin(cos(2-2))", (1 as f64).sin());
}

#[test]
fn _1st_deg_poly() {
    let mut env = Env::prelude();
    let _ = run("7x = 14", &mut env);
    assert_eq!(
        env.find_value("x"),
        Ok(cwim::interpreter::Expr::Literal(Number::scalar(2.)))
    );
}

// TODO: Test assignments more.
#[test]
fn _assignment() {
    let mut env = Env::prelude();
    let _ = run("x = 6", &mut env);
    assert_eq!(
        env.find_value("x"),
        Ok(cwim::interpreter::Expr::Literal(Number::scalar(6.)))
    );
    let _ = run("7z+5z = 12", &mut env);
    assert_eq!(
        env.find_value("z"),
        Ok(cwim::interpreter::Expr::Literal(Number::scalar(1.)))
    );
}

#[test]
fn _assignment2() {
    let mut env = Env::prelude();
    let _ = run("(sin pi/2)y=1", &mut env);
    assert_eq!(
        env.find_value("y"),
        Ok(cwim::interpreter::Expr::Literal(Number::scalar(1.)))
    );
}
