use cwim::interpreter::*;
use inquire::Text;

fn main() {
    loop {
        let line = Text::new("").prompt().expect("invalid cwim");
        match run(&line) {
            Ok(result) => println!("{}", result),
            Err(msg) => eprintln!("{}", msg),
        }
    }
}
