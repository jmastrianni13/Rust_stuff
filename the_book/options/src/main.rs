use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1"); // sets env var for this program only

    let five = optional_u8(Some(5));
    println!("five == {:#?}", five);
    let five = five.unwrap();
    println!("five unwrapped == {:#?}", five);

    let none = optional_u8(None); // this is already unwrapped
    println!("none == {:#?}", none);
    let none = none.unwrap(); // causes my first panic :) 
    println!("none unwrapped == {:#?}", none);
}

fn optional_u8(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(i) => Some(i), // has to be Some
    }
}
