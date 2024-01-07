pub fn main() {
    let mut seq: Vec<char> = "abc".chars().collect();
    permute(&mut seq, 0);
}

fn permute(seq: &mut [char], i: usize) {
    if i == seq.len() {
        println!("{}", seq.iter().collect::<String>());
        return;
    }

    for s in i..seq.len() {
        seq.swap(i, s);
        permute(seq, i + 1);
        seq.swap(i, s);
    }
    return;
}
