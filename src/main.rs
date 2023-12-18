use cwim::env::*;
use cwim::interpreter::*;
use inquire::Text;

fn main() {
    let mut env = Env::prelude();
    while let Ok(line) = Text::new("").prompt() {
        match run(&line, &mut env) {
            Ok(result) => {
                println!("{}", result);
                env.assign("ans".to_owned(), Variable::Value(result));
            }
            Err(msg) => eprintln!("{}", msg),
        }
    }
}
