use cwim::interpreter::*;
use inquire::Text;

fn main() {
    while let Ok(line) = Text::new("").prompt() {
        match run(&line) {
            Ok(result) => println!("{}", result),
            Err(msg) => eprintln!("{}", msg),
        }
    }
}
