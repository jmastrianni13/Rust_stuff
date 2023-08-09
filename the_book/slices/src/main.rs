fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
 
     
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}

fn second_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    let mut first_space: usize = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
           if first_space == 0 {
               first_space = i+1
           }
           else {
               return &s[first_space..i];
           }
        }
    }
    
    return &s[first_space..];

}

fn main() {
    let mut s = String::from("Hello, world!");
    let word = first_word(&s);
    println!("s = {}", s);
    println!("word = {}", word);
    s.clear(); // this sets String to ""
    // problem here is word still exists and is totally independent of s
    let s = String::from("hello world");
    // let hello = &s[0..5]; // == &s[..5]
    // let world = &s[6..11]; // == &s[6..]

    let word = first_word_slice(&s);

    println!("the first word is: {}", word);

    let word2 = second_word_slice(&s);
    println!("the second word is: {}", word2);

}
