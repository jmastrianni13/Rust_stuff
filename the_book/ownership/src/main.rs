// ownership
// Rust uses a stack and a heap to store data in memory
// The stack stores data with a known, fixed size
// The heap stores data that can vary or has an unknown size 
fn demo_scope() {
    println!("--- demo_scope ---");
    let s = "hello"; // s1

    {
        // in this inner scope s1 is not seen and s is undefined
        let s = "hello again"; // s2
        println!("{}", s)
     
    } // leaving this scope, s (s2) is dropped

    println!("{}", s) // this s refers to the s above, s1

}

fn demo_stack() {
    println!("--- demo_stack ---");
    let x: i32 = 5;
    println!("x = {}", x);
    let y = x; 
    println!("y = {}", y);
    println!("x still = {}", x);
} 

fn demo_mutable() {
    println!("--- demo_mutable---");
    let mut x = 5;
    println!("x = {}", x);
    let mut y = x; // not a move? no, it is not
    println!("y = {}", y);
    x += 2;
    y += 1;
    println!("x = {}", x); // on stack because exists? yes, on the stack
    println!("y = {}", y); // on stack because exists? yes, on the stack

    let mut s = String::from("hello");
    println!("s = {}", s);
    s.push_str(", world!");
    println!("s = {}", s);
    let t = s; // this is a move from s to t, on heap? yes, on the heap
    println!("t = {}", t);
    // println!("s = {}", s)?; // will raise erro because s is no longer valid
    
    let mut s1 = String::from("hello");
    let mut s2 = s1.clone(); // not a move
    println!("s1 = {}, s2 = {}", s1, s2);
    s1.push_str(", world!");
    s2.push_str(", again!");
    println!("s1 = {}, s2 = {}", s1, s2);

}

fn main() {
    demo_scope();
    demo_stack();
    demo_mutable();
}

