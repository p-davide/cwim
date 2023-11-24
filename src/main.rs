use cwim::parser::*;
use cwim::interpreter::*;

fn run(text: &str) -> Option<f64> {
    let tks = parse(text)?;
    let exprs = understand(tks)?;
    let s = shuntingyard(exprs)?;
    eval(s)
}

fn main() {
    //let expr1 = "(234 + 400) * 8";
    //let expr2 = "[234x + 400 1222] * [8; 10] # this is a comment";
    let expr3 = "234*5+7*8/0";
    println!("{:?}", run(expr3));
}
