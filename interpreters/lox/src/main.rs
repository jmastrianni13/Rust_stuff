use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(args[1].as_str());
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path)
        .expect("could not read file");
}

fn run_prompt() {

}

