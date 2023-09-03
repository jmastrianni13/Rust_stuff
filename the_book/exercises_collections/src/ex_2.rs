pub fn get_pig_latin(text: &str) {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    let first_char = text.chars().next().unwrap();
    let mut latin_text = String::from(&text[1..]);
    
    if vowels.contains(&first_char) {
        latin_text = String::from(first_char) + &latin_text;
        latin_text.push_str("-hay");
    }
    else {
        let mut first_char = String::from(first_char);
        first_char = "-".to_owned() + &first_char;
        first_char.push_str("ay");
        latin_text.push_str(&first_char);
    }
    
    println!("{text} => {latin_text}");
}
