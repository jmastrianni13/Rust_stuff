use std::io;
fn main() {
    println!("Welcome to the simple shell");

    let path = std::env::var("PATH").unwrap_or_else(|_| String::new());
    println!("{path}");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        println!("{line}");
    }
}
