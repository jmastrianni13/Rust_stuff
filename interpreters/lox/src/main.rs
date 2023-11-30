mod environment;
mod expr;
mod interpreter;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod tests;

use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        match run_file(&args[1]) {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("ERROR: {}", msg);
                process::exit(1);
            }
        }
    } else if args.len() == 3 && args[1] == "e" {
        match run_string(&args[2]) {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("ERROR: {}", msg);
                process::exit(1);
            }
        }
    } else if args.len() == 1 {
        match run_prompt() {
            Ok(_) => process::exit(0),
            Err(msg) => {
                println!("ERROR: {}", msg);
                process::exit(1);
            }
        }
    } else {
        println!("Usage: jlox [script]");
        process::exit(64);
    }
}

pub fn run_file(path: &str) -> Result<(), String> {
    let mut interp = interpreter::Interpreter::new();
    match fs::read_to_string(path) {
        Err(msg) => return Err(msg.to_string()),
        Ok(contents) => return run(&mut interp, &contents),
    }
}

pub fn run_string(contents: &str) -> Result<(), String> {
    let mut interpreter = interpreter::Interpreter::new();
    return run(&mut interpreter, contents);
}

fn run_prompt() -> Result<(), String> {
    let mut interp = interpreter::Interpreter::new();
    let mut buffer = String::new();
    loop {
        print!("> ");
        match io::stdout().flush() {
            Ok(_) => (),
            Err(_) => return Err("could not flush stdout".to_string()),
        }

        let stdin = io::stdin();
        let mut handle = stdin.lock();
        let current_length = buffer.len();
        match handle.read_line(&mut buffer) {
            Ok(n) => {
                if n < 1 {
                    return Ok(());
                }
            }
            Err(_) => return Err("count not read line".to_string()),
        }

        println!("got: {}", &buffer[current_length..]);
        match run(&mut interp, &buffer[current_length..]) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn run(interp: &mut interpreter::Interpreter, contents: &str) -> Result<(), String> {
    let mut scanner = scanner::Scanner::new(contents);
    scanner.scan_tokens()?;
    let tokens = scanner.tokens;

    let mut parser = parser::Parser::new(tokens);
    let statements = parser.parse()?;

    interp.interpret(statements.iter().collect())?;
    return Ok(());
}
