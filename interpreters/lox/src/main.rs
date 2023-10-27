mod scanner;
use crate::scanner::*;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("ERROR: {}", msg);
                process::exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("ERROR: {}", msg);
                process::exit(1);
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&contents),
    }
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("could not flush stdout".to_string()),
        }

        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        match handle.read_line(&mut buffer) {
            Ok(_) => {
                buffer = buffer.trim().to_string();
                if buffer.len() == 0 {
                    return Ok(());
                }
            }
            Err(_) => return Err("count not read line".to_string()),
        }
        println!("got: {}", buffer);
        match run(&buffer) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn run(contents: &str) -> Result<(), String> {
    let scanner = Scanner::new(contents);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    return Ok(());
}

