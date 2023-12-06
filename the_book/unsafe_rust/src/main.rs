use std::ops::Add;
use std::slice;

fn main() {
    demo_raw_pointers();
    demo_unsafe_functions();
    demo_unsafe_abstraction();
    demo_extern();
    demo_mut_static_var();
    demo_counter();
    demo_op_overload();
    demo_ambiguous_methods();
}

fn demo_raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("num is {num} at {:p}", &num); // :p prevent println! automatically dereferencing
    println!("r1 is {r1:?} at {:p}", &r1);
    println!("r2 is {r2:?} at {:p}", &r2);

    // no good reason to write code like the following
    let address = 0x012345usize; // arbitrary address
    let r = address as *const i32;
    println!("r is {r:?} at {:p}", &r);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    demo_mut_static_var();
}

fn demo_unsafe_functions() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

fn demo_unsafe_abstraction() {
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            return (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            );
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    println!("left of split: {:?}", left);
    println!("right of split: {:?}", right);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn demo_extern() {
    unsafe {
        println!("Absolute value of -3 accroding to C: {}", abs(-3));
    }
}

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn demo_mut_static_var() {
    add_to_counter(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        return Counter { count: 0 };
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        } else {
            return None;
        }
    }
}

fn demo_counter() {
    let mut counter = Counter::new();
    let mut c;
    loop {
        c = counter.next();
        match c {
            None => break,
            _ => println!("{}", c.unwrap()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn demo_op_overload() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };

    let p3 = p1 + p2;

    assert_eq!(p3, Point { x: 3, y: 3 });
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        return Millimeters(self.0 + (other.0 + 1000));
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        return String::from("Spot");
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        return String::from("puppy");
    }
}

fn demo_ambiguous_methods() {
    let person = Human;
    person.fly(); // calls fly method directly implemented on Human
    Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);

    // because Animal does not take self, different syntax is required to call the method
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // <Type as Trait>::function()
}
