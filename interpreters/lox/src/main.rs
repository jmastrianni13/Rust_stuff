use std::env;
use std::fs;
use std::io;
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
    print!("> ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err("count not read line".to_string()),
    }
    println!("got: {}", buffer);
    return Ok(());
}

fn run(contents: &str) -> Result<(), String> {
    return Err("not implemented".to_string());
}

