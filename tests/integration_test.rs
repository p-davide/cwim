use cwim::interpreter::run;

fn _test_run(text: &str, expected: f64) {
    assert_eq!(run(text), Ok(expected));
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
fn _run_with_spaces_2() {
    _test_run(
        "234*5+7*8-18 ^ 3",
        (234. * 5. + 7. * 8 as f64 - 18.).powf(3.),
    );
}

#[test]
fn _run_with_spaces_3() {
    let text = "234 *5+7*8-18 ^ 3";
    assert_eq!(run(text), Ok(234. * (5. + 7. * 8. - 18 as f64).powf(3.)));
}

#[test]
fn _run_with_spaces_4() {
    assert_eq!(
        run("234 * 5+7*8-18 ^ 3"),
        Ok(234. * (5. + 7. * 8. - 18 as f64).powf(3.))
    );
}

#[test]
fn _run_with_parens_3() {
    assert_eq!(
        run("234 *(5+7*8-18) ^ 3"),
        Ok(234. * (5. + 7. * 8. - 18 as f64).powf(3.))
    );
}

#[test]
fn _unmatched_parens() {
    assert_eq!(run("4)"), Err("unmatched )".to_owned()));
}

#[test]
fn _spaces_brackets_and_negations() {
    _test_run(" -(6) * -(6)", 36.);
}
