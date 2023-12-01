use cwim::interpreter::*;
use inquire::Text;

fn main() {
    loop {
        let line = Text::new("").prompt().expect("invalid cwim");
        if let Ok(result) = run(&line) {
            println!("{}", result)
        } else {
            eprintln!("invalid cwim")
        }
    }
}
