fn main() {
    demo_if_let_exp();
    demo_while_let();
    demo_for_loop();
    demo_matching();
    demo_destructuring();
}

fn demo_if_let_exp () {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using you favorite color, {color}, as the background.");
    } else if is_tuesday {
        println!("Tuesday is greenday!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn demo_while_let () {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn demo_for_loop () {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NestedMessage {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn demo_matching() {
    // literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }

    // named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // this is not same y as declared above (cannot
                                                 // use y above here)
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // multiple patterns
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything else"),
    }

    // ranges
    let x = 5;

    match x {
        1..=5 =>  println!("one through five"),
        _ => println!("not one through five"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ACSII letter"),
        _ => println!("something else"),
    }

    // ignoring values with _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let s = Some(String::from("Hello!"));

    if let Some(_) = s { // _ does not bind, so s is not moved
        println!("found a string");
    }

    println!("{:?}", s);

    // ignoring parts of a value with ..
    let origin = Point3d { x: 0, y: 0, z: 0};

    match origin {
        Point3d { x, ..} => println!("x is {}", x),
    }

    match numbers { // defined above
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"), // y is not shadowed anymore
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

}

fn demo_destructuring() {
    let p = Point{ x: 0, y: 7};

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point{x, y: 0} => println!("On the x axis at {x}"),
        Point{x: 0, y} => println!("On the y axis at {y}"),
        Point{x, y} => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has not data to destructure.");
        }
        Message::Move{x, y} => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");
        }
    }

    let msg = NestedMessage::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        NestedMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        NestedMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, and value {v}");
        }
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet = {feet}, inches = {inches}, x = {x}, y = {y}");
}

