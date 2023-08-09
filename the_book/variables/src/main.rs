const MULTIPLIER: u32 = 5;
fn main()  {
    let mut x = 5 * MULTIPLIER; // constants example
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 7;
    {
        let y = y * 2;
        println!("In here, y is: {y}")
    }

    println!("Out here, y is: {y}")
}

