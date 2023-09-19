use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // simple_panic();
    // vec_overread_panic();
    // get_missing_file();
    // get_missing_file_no_match();
    // get_unwrapped_file();
    // get_unwrapped_file_w_expect();
    // read_username_from_file();
    let greeting_file = File::open("hello.txt")?;
    return Ok(()) 
}

fn simple_panic() {
    panic!("crash and burn!");
}

fn vec_overread_panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn get_missing_file() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn get_missing_file_no_match() {
    // same as get_missing_file absent use of match statements
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn get_unwrapped_file() {
    let greeting_file = File::open("hello.txt").unwrap();
}

fn get_unwrapped_file_w_expect() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    return match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };
}

fn read_username_from_file_wq() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // the ? replaces the match
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

