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
            env.assign("ans".to_owned(), result);
        }
        Err(msg) => eprintln!("{}", msg),
    }
}

fn repl() -> Result<()> {
    let mut env = Env::prelude();
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                run_line(&line, &mut env);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
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
