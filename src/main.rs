use cwim::interpreter::*;
use inquire::Text;

fn main() {
    while let Ok(line) = Text::new("").prompt() {
        match run(&line, &cwim::env::Env::std()) {
            Ok(result) => println!("{}", result),
            Err(msg) => eprintln!("{}", msg),
        }
    }
}
