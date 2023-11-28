use cwim::interpreter::*;
use cwim::parser::*;
use cwim::prioritize::*;
use inquire::Text;

fn run(text: &str) -> Parsed<f64> {
    let tks = parse(text)?;
    let parens = prioritize(tks);
    let exprs = understand(parens)?;
    let s = shuntingyard(exprs)?;
    eval(s)
}

#[cfg(test)]
mod test {
    use super::*;
    fn _test_run(text: &str, expected: f64) {
        assert_eq!(run(text), Ok(expected));
    }

    #[test]
    fn _parse() {
        let actual = parse("234*5+7*8-18^3").map(|ts| ts.iter().map(|t| t.lexeme).collect());
        assert_eq!(
            actual,
            Ok(vec![
                "234", "*", "5", "+", "7", "*", "8", "-", "18", "^", "3",
            ])
        );
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
}
fn main() {
    //let expr1 = "(234 + 400) * 8";
    //let expr2 = "[234x + 400 1222] * [8; 10] # this is a comment";
    loop {
        let line = Text::new("").prompt().expect("invalid cwim");
        if let Ok(result) = run(&line) {
            println!("{}", result)
        } else {
            eprintln!("invalid cwim")
        }
    }
}
