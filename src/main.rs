use std::io;
use std::io::IsTerminal;

use cwim::env::*;
use cwim::interpreter::*;
use inquire::Text;

fn run_line(line: &str, env: &mut Env) {
    match run(&line, env) {
        Ok(result) => {
            println!("{}", result);
            env.assign("ans".to_owned(), Variable::Value(result));
        }
        Err(msg) => eprintln!("{}", msg),
    }
}

fn repl() {
    let mut env = Env::prelude();
    while let Ok(line) = Text::new("").prompt() {
        run_line(&line, &mut env);
    }
}

fn main() {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        repl()
    } else {
        let mut env = Env::prelude();
        for line in stdin.lines() {
            run_line(&line.expect("no line found"), &mut env);
        }
    }
}
