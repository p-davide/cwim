use cwim::interpreter::*;
use cwim::parser::*;

fn run(text: &str) -> Option<f64> {
    let tks = parse(text)?;
    let exprs = understand(tks)?;
    let s = shuntingyard(exprs)?;
    eval(s)
}

fn _test_run(text: &str, expected: f64) {
    assert_eq!(run(text), Some(expected));
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
    assert_eq!(run("234 *5+7*8-18 ^ 3"), None);
}
fn main() {
    //let expr1 = "(234 + 400) * 8";
    //let expr2 = "[234x + 400 1222] * [8; 10] # this is a comment";
}
