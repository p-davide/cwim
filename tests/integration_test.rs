use cwim::env::*;
use cwim::interpreter::run;
fn _test_run(text: &str, expected: f64) {
    assert_eq!(run(text, &mut Env::prelude()), Ok(expected));
}

#[test]
fn _run_basic() {
    _test_run("234*5+7*8-18^3", 234. * 5. + 7. * 8. - (18 as f64).powf(3.));
}

#[test]
fn _run_with_spaces_1() {
    _test_run(
        "234 * 5+7*8-18 ^ 3",
        234. * (5. + 7. * 8 as f64 - 18.).powf(3.),
    );
}

#[test]
fn _minus() {
    _test_run("3 - 2 - 1", 0.);
    _test_run("3-2-1", 0.);
}

#[test]
fn _plus_minus() {
    _test_run("5-+5", 0.);
    _test_run("5+-5", 0.);
}

#[test]
fn _run_with_spaces_2() {
    _test_run(
        "234*5+7*8-18 ^ 3",
        (234. * 5. + 7. * 8 as f64 - 18.).powf(3.),
    );
}

#[test]
fn _reg_1() {
    _test_run("2(log10 * 7)", 14.);
}

#[test]
fn _a() {
    _test_run("6+1*9", 6. + 1. * 9.);
    _test_run("6 + 1 * 9", 6. + 1. * 9.);
    _test_run("5 + 6 + 1 * 9", 5. + 6. + 1. * 9.);
    _test_run(
        "2 ^ 4 * 5 + 6 + 1 ^ 9",
        2f64.powf(4.) * 5. + 6. + 1f64.powf(9.),
    )
}

#[test]
fn _cos_2pi() {
    _test_run("cos 2pi", 1.)
}

#[test]
fn _run_with_spaces_3() {
    _test_run("234 *5+7*8-18 ^ 3", 234. * (5. + 7. * 8. - 18f64).powf(3.));
}

#[test]
fn _run_with_spaces_4() {
    _test_run(
        "234 * 5+7*8-18 ^ 3",
        234. * (5. + 7. * 8. - 18 as f64).powf(3.),
    );
}

#[test]
fn _implied_multiplication() {
    _test_run("2(8)", 16.);
    _test_run("(+3+5)", 8.);
}

#[test]
fn _implied_multiplication_2() {
    _test_run("2(+3+5)", 16.);
    _test_run("2 (+3+5)", 16.);
}

#[test]
fn _implied_multiplication_3() {
    _test_run("2 3", 6.);
}

#[test]
fn _run_with_parens_3() {
    _test_run(
        "234 *(5+7*8-18) ^ 3",
        234. * (5. + 7. * 8. - 18 as f64).powf(3.),
    );
}

#[test]
fn _just_a_number() {
    _test_run("3", 3.);
    _test_run("(3)", 3.);
    _test_run(" 3", 3.);
    _test_run(" (3)", 3.);
    _test_run(" 3 ", 3.);
    _test_run(" (3 )", 3.);
}

#[test]
fn _unmatched_parens() {
    assert_eq!(run("4)", &mut Env::prelude()), Ok(4.));
}

#[test]
fn _spaces_brackets_and_negations() {
    _test_run(" -(6) * -(6)", 36.);
}

#[test]
fn _unary_ordering() {
    _test_run("cos2pi   ", (2. * std::f64::consts::PI).cos());
    _test_run("cos 2pi  ", (2. * std::f64::consts::PI).cos());
    _test_run("cos2 pi  ", 2f64.cos() * std::f64::consts::PI);
    _test_run("cos 2 pi ", (2. * std::f64::consts::PI).cos());
}

#[test]
fn _double_unary() {
    _test_run("cos cos 2-2", (1 as f64).cos());
    _test_run("cos(cos 2-2)", (1 as f64).cos());
    _test_run("cos(cos(2-2))", (1 as f64).cos());
    assert_eq!(
        1. - run("acos(cos(3-2))", &mut Env::prelude()).unwrap() < std::f64::EPSILON,
        true
    );
    assert_eq!(
        1. - run("acosh cosh(3-2)", &mut Env::prelude()).unwrap() < std::f64::EPSILON,
        true
    );
}

#[test]
fn _1st_deg_poly() {
    let mut env = Env::prelude();
    let _ = run("7x = 14", &mut env);
    assert_eq!(
        env.find_value("x"),
        Ok(cwim::interpreter::Expr::Literal(2.))
    );
}

#[test]
fn _fwefw() {
    let mut env = Env::prelude();
    let _ = run("7y+5y = 12", &mut env);
    assert_eq!(
        env.find_value("y"),
        Ok(cwim::interpreter::Expr::Literal(1.))
    );
}

// TODO: Test assignments more.
#[test]
fn _assignment() {
    let mut env = Env::prelude();
    let _ = run("x = 6", &mut env);
    assert_eq!(
        env.find_value("x"),
        Ok(cwim::interpreter::Expr::Literal(6.))
    );
}
