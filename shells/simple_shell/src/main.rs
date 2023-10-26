use std::(io, path::PathBuf);

fn main() {
    println!("Welcome to the simple shell");
    let vars: Vec<String> = std::env::args().collect();
    let path = std::env::var("PATH").unwrap_or_else(|_| String::new());
    println!("{path}");
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();
        println!("{line}");
    }
}

fn run_process(vars: &Vec<String>, command: &str) -> Result<(), ()> {
    return Ok(());
}

fn find_binary(command: &str) -> Result<PathBuf, std::io::Error> {
    fn search(command: &str, path: &Path) -> Result<(), std::io::Error> {
        for entry in std::fs::read_dir(path)? {
            if let Ok(entry) = entry {
                if let Ok(met) = entry.metadata() {
                    if met.is_file() || met.is_symlink() {
                        if let Some(name) = entry.path().file_name() {
                            if name == command {
                                if met.is_symlink() {
                                    panic!("Running symlinks not supported");
                                }
                                return Ok(());
                            }
                        }
                    }
                 }
            }
        }
        Err(std::io::ErrorKind::NotFound.into())
    };
}
