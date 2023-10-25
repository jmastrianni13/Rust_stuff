use std::io;
fn main() {
    println!("Welcome to the simple shell");

    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        println!("{line}");
    }
}
