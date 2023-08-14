use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10); // since i32 has copy trait, values are copied to map
    scores.insert(String::from("red"), 50);
    
    let team_name = String::from("blue");
    // get returns Option<&V>.  copied returns the actual value so Option<i32> here
    // unwrap_or will set score to 0 if &team_name is not in scores
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("blue team score == {score}");

    print_hashmap(&scores);

    let field_name = String::from("favorite color");
    let field_value = String::from("green");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // since field_name, field_value do not have copy, they are moved into the map
    println!("map == {:?}", map);
    scores = update_scores_hashmap(scores, String::from("blue"), 75);
    scores = update_scores_hashmap(scores, String::from("lightish red"), -4000);
    print_hashmap(&scores);

    // only add a key value pair if key does not exist
    scores.entry(String::from("pink")).or_insert(3);
    scores.entry(String::from("red")).or_insert(45);
    print_hashmap(&scores);

    let sentence = "hello world wonderful world";

    let word_count = get_word_count(sentence);
    println!("{:?}", word_count);
}

fn print_hashmap(hp: &HashMap<String, i32>) {
    for (key, value) in hp {
        println!("{key}: {value}");
    }
}
    
fn update_scores_hashmap(mut hp: HashMap<String, i32>, team: String, score: i32) -> HashMap<String, i32> {
    hp.insert(team, score);
    return hp;
}

fn get_word_count(text: &str) -> HashMap<&str, i32> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // or_insert returns mutable ref (&mut V) so need to dereference to mutate
    }
    return map;
}

