use std::io;
use std::io::IsTerminal;

use cwim::env::*;
use cwim::interpreter::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn run_line(line: &str, env: &mut Env) {
    match run(line, env) {
        Ok(result) => {
            println!("{}", result);
            env.assign("ans".to_owned(), &result);
        }
        Err(msg) => eprintln!("{}", msg),
    }
}

fn repl() -> Result<()> {
    let mut env = Env::prelude();
    let mut rl = DefaultEditor::new()?;
    let history = ".cwim_history";
    if rl.load_history(history).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("cwim> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                run_line(&line, &mut env);
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(history)
}

fn main() {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        let _ = repl();
    } else {
        let mut env = Env::prelude();
        for line in stdin.lines() {
            run_line(&line.expect("no line found"), &mut env);
        }
    }
}
