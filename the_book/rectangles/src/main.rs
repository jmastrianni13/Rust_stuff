fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1: (u32, u32) = (30, 50);

    println!(
        "using tuples, the area of the rectangle is {} square pixels",
        area_tuples(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("prettyprint rect1 is {:#?}", rect1);

    println!(
        "using area func and Rectangle Structs, the area of the rectangle is {} square pixels",
        area_struct(&rect1)
    ); // use a reference so that main continues to own rect1

    // to debug, use the !dbg macro
    dbg!(&rect1); // pass a ref so dbg does not take ownership

    println!(
        "using struct method, the area of the rectangle is {} square pixels",
        rect1.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("does rect2 fit in rect1? {}", rect1.can_hold(&rect2));
    println!("does rect3 fit in rect1? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::create_square(4);
    println!("sq is {:#?}\n  with area {}", sq, sq.area())

}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 { // borrow Rectangle and its attributes
    return rectangle.width * rectangle.height;
}

#[derive(Debug)] // needed to print Rectangle
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle { // impl means implementation.  these are methods aka associated functions
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

impl Rectangle { // not an associated function because it does not take self as a param
    fn create_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
