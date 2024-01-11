pub fn main() {
    let mut seq: Vec<char> = "abc".chars().collect();
    print_permute(&mut seq, 0);
    let mut my_perms: Vec<String> = vec![];
    get_permute(&mut seq, 0, &mut my_perms);
    println!("{:?}", my_perms);
}

fn print_permute(seq: &mut [char], i: usize) {
    if i == seq.len() {
        println!("{}", seq.iter().collect::<String>());
        return;
    }

    for s in i..seq.len() {
        seq.swap(i, s);
        print_permute(seq, i + 1);
        seq.swap(i, s);
    }
    return;
}

fn get_permute(seq: &mut [char], i: usize, result: &mut Vec<String>) {
    if i == seq.len() {
        let perm = seq.iter().collect::<String>();
        result.push(perm);
        return;
    }

    for s in i..seq.len() {
        seq.swap(i, s);
        get_permute(seq, i + 1, result);
        seq.swap(i, s);
    }
    return;
}
