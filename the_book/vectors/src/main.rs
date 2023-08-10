fn main() {

    let v: Vec<i32> = Vec::new(); // new immutable Vec
    println!("v == {:?}", v);

    let mut v: Vec<i32> = Vec::new(); // new mutable Vec
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v == {:?}", v);

    let mut v = Vec::new(); // type hint omitted
    v.push(3); // infers type from this value
    v.push(2);
    v.push(1);

    println!("v == {:?}", v);

    let v = vec![4, 5, 6]; // vec! is a macro, i32 is inferred from initial values
    println!("v == {:?}", v);
    
    let value = v[0]; // gets the value at index 0, v is mutable
    println!("value == {value}");
    println!("v still == {:?}", v);
    

    let i = 0;
    let value_by_index: &i32 = get_value_by_index(&v,i); // gets a reference to the value at index i, value_by_index is immutable and prevents mutating v
    println!("value_by_index == {value_by_index}");

    let i = 1;
    let value_by_get: Option<&i32> = v.get(i);
    println!("value_by_get == {:?}", value_by_get);

    // to unwrap value_by_get, can use match
    match value_by_get {
        Some(value_by_get) => println!("value_by_get value is {value_by_get}"),
        None => println!("value_by_get is None")
    };

    // looking up an index > last index of Vec
    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = v.get(100);
    // let does_not_exist = &v[100]; // raises an index out of bounds pani

    println!("does_not_exist == {:?}", does_not_exist);

    iterate_over_vec(v);
    // println!("v? {:?}", v);

    let v = vec![0, 2, 4, 6];
    let v = update_vec(v);
    println!("v == {:?}", v);

    // to create a vec with multiple types, use an enum
    let multivec = vec![
        MultipleTypes::Int(3),
        MultipleTypes::Float(3.14),
        MultipleTypes::Text(String::from("text")),
    ];
    println!("multivec = {:?}", multivec);
}

#[derive(Debug)]
enum MultipleTypes {
    Int(i32),
    Float(f64),
    Text(String),
}

fn get_value_by_index(v: &Vec<i32>, i: usize) -> &i32 {
    let value: &i32 = &v[i];
    return value;
}

fn iterate_over_vec(v: Vec<i32>) {
    for i in v { // this for loop takes ownership of
        println!("{i}");
    }; // v is dropped
}

fn update_vec(mut v: Vec<i32>) -> Vec<i32> {
    for i in &mut v {
        *i += 50;
    }
    return v;
}

