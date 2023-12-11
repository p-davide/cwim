use cwim::env::*;
use cwim::interpreter::run;
fn _test_run(text: &str, expected: f64) {
    assert_eq!(run(text, &Env::std()), Ok(expected));
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
fn _run_with_spaces_2() {
    _test_run(
        "234*5+7*8-18 ^ 3",
        (234. * 5. + 7. * 8 as f64 - 18.).powf(3.),
    );
}

#[test]
fn _a() {
    _test_run("2 ^ 4 * 5 + 6 + 1 ^ 9", 2f64.powf(4.) * 5. + 6. + 1f64.powf(9.))
}

#[test]
fn _run_with_spaces_3() {
    _test_run(
        "234 *5+7*8-18 ^ 3",
        234. * (5. + 7. * 8. - 18f64).powf(3.),
    );
}

#[test]
fn _run_with_spaces_4() {
    _test_run(
        "234 * 5+7*8-18 ^ 3",
        234. * (5. + 7. * 8. - 18 as f64).powf(3.),
    );
}

#[test]
fn _run_with_parens_3() {
    _test_run(
        "234 *(5+7*8-18) ^ 3",
        234. * (5. + 7. * 8. - 18 as f64).powf(3.),
    );
}

#[test]
fn _unmatched_parens() {
    assert_eq!(run("4)", &Env::std()), Err("unmatched )".to_owned()));
}

#[test]
fn _spaces_brackets_and_negations() {
    _test_run(" -(6) * -(6)", 36.);
}

#[test]
fn _double_unary() {
    _test_run("cos cos 0", (1 as f64).cos());
    _test_run("cos(cos 0)", (1 as f64).cos());
    _test_run("cos(cos(0))", (1 as f64).cos());
    assert_eq!(
        1. - run("acos(cos(1))", &Env::std()).unwrap() < std::f64::EPSILON,
        true
    );
    assert_eq!(
        1. - run("acosh cosh(1)", &Env::std()).unwrap() < std::f64::EPSILON,
        true
    );
}
