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
        Some(y) => println!("Matched, y = {y}"), // this is not same y as declared above
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
}

