use std::collections::HashMap;

pub fn get_median(numbers: &Vec<i32>) -> i32 {
    let size = numbers.len();
    let mut sorted_numbers: Vec<i32> = numbers.to_vec();
    sorted_numbers.sort();
    let mid_point: usize = (size / 2).try_into().unwrap();
    return sorted_numbers[mid_point];
}

pub fn get_mode(numbers: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for number in numbers {
        let count = map.entry(number).or_insert(0); // gives mutable reference tovalue associated with entry
        *count += 1; // dereferences entry so it can be modified 
    }

    let mut max_val: i32 = 0;
    let mut mode: i32 = 0;

    for (key, value) in map {
        if value > max_val {
            mode = *key;
            max_val = value;
        }
    }
    return mode;
}

