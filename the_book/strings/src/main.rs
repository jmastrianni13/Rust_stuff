fn main() {
    // double quotes for strings, single for chars
    // "string", 'char'

    let mut s = String::new(); // can load data into this empty string

    println!("s == {:?}", s);

    let data = "initial contents";

    s = data.to_string();

    println!("s == {:?}", s);

    // some ways to make a string
    let _s = data.to_string(); // _ removes warnings these statements would raise
    let _s = "initial contents".to_string();
    let _s = String::from("initial contents");

    let message = concat_push_str("hello", " world");
    println!("message == {:?}", message);
    let message = concat_plus_str("hello", " world");
    println!("message == {:?}", message);
    let message = concat_plus_str("hello", " world");
    println!("message == {:?}", message);
    let message = concat_format_str("hello", "world");
    println!("message == {:?}", message);

    let s = "hello";
    print_chars(&s);
    print_bytes(&s);
}

fn concat_push_str(s1: &str, s2: &str) -> String {
    let mut concat_str = String::from(s1);
    concat_str.push_str(s2);
    concat_str.push('!'); // can use push for adding single characters
    return concat_str;
}

fn concat_plus_str(s1: &str, s2: &str) -> String {
    let mut concat_str = String::from(s1);
    concat_str += s2;
    return concat_str;
}

fn concat_format_str(s1: &str, s2: &str) -> String {
    let concat_str = format!("{s1} {s2}");
    return concat_str;
}

fn print_chars(s: &str) {
    for c in s.chars() {
        println!("{c}")
    }
}

fn print_bytes(s: &str) {
    for b in s.bytes() {
        println!("{b}");
    }
}

