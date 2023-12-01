use cwim::interpreter::*;
use inquire::Text;

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
