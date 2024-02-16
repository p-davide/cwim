use std::fmt::Display;
use std::io;
use std::io::IsTerminal;
use std::str::FromStr;

use cwim::env::*;
use cwim::interpreter::*;
use num_traits::real::Real;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn run_line<N: Display + Real + std::fmt::Debug + FromStr>(line: &str, env: &mut Env<N>)
where
    <N as FromStr>::Err: std::fmt::Debug,
{
    match run(line, env) {
        Ok(result) => {
            println!("{}", result);
            env.assign("ans".to_owned(), result);
        }
        Err(msg) => eprintln!("{}", msg),
    }
}

fn repl<N: Display + Real + std::fmt::Debug + FromStr>() -> Result<()> where
    <N as FromStr>::Err: std::fmt::Debug,{
    let mut env = Env::<N>::prelude();
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
    Ok(())
}

fn main() {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        let _ = repl::<f64>();
    } else {
        let mut env = Env::<f64>::prelude();
        for line in stdin.lines() {
            run_line(&line.expect("no line found"), &mut env);
        }
    }
}
