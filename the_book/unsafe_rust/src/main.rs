fn main() {
    demo_raw_pointers();
    demo_unsafe_functions();
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
}

fn demo_unsafe_functions() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}
