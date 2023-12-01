use std::slice;

fn main() {
    demo_raw_pointers();
    demo_unsafe_functions();
    demo_unsafe_abstraction();
    demo_extern();
    demo_mut_static_var();
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
