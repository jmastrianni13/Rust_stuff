fn get_len_borrow(s: &String) -> usize { // & makes a ref or pointer to s
    // since it is borrowed, s is immutable
    return s.len();
} // s is borrowed and not dropped

fn get_len_move(mut s: String) -> usize { // s moves to this func
    s.push_str("!");
    return s.len();
} // s is dropped

fn get_len_mut_borrow(s: &mut String) -> usize { // &mut allows mutation of String
    s.push_str(" mutation!");
    return s.len();
}

fn main() {
    let s1 = String::from("hello"); // String is stored on heap
    let len = get_len_borrow(&s1); // let get_len 'borrow' s1 by using a ref
    println!("The length of '{}' is {}", s1, len);

    let s2 = String::from("world");
    let len = get_len_move(s2); // s2 moves to get_len_move
    println!("The len of s2 is {}", len);

    let mut s3 = String::from("hello");
    let len = get_len_mut_borrow(&mut s3); // this will mutate s3
    println!("The len of '{}' is {}", s3, len);
}

