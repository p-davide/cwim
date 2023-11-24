use cwin::parser::*;
use cwin::interpreter::*;

fn run(text: &str) -> Option<f64> {
    let tks = parse(text)?;
    let exprs = understand(tks)?;
    let s = shuntingyard(exprs)?;
    if let Some(n) = eval(s) {
        Some(n)
    } else {
        None
    }
}

fn main() {
    //let expr1 = "(234 + 400) * 8";
    //let expr2 = "[234x + 400 1222] * [8; 10] # this is a comment";
    let expr3 = "234*5+7*8/0";
    println!("{:?}", run(expr3));
}
