fn main() {
    let i32_list = vec![34, 50, 25, 100, 65];
    let largest_i32 = get_largest_i32(&i32_list);
    println!("largest_i32 == {largest_i32}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = get_largest_char(&char_list);
    println!("largest_char == {largest_char}");

    let generic_list = vec![34, 50, 25, 100, 65];
    let largest_generic = get_largest_generic(&generic_list);
    println!("largest_generic == {largest_generic}");

    let generic_list = vec!['y', 'm', 'a', 'q'];
    let largest_generic = get_largest_generic(&generic_list);
    println!("largest_generic == {largest_generic}");

    let int_point = Point{x:5, y:10};
    println!("int_point == {:?}", int_point);
    println!("int_point.x == {}", int_point.x());
    let float_point = Point{x:5.0, y:10.0};
    println!("float_point == {:?}", float_point);
    println!("float_point.distance_from_origin == {}", float_point.distance_from_origin());
    let int_float_point = MultivalPoint{x:5, y:10.0};
    println!("int_float_point == {:?}", int_float_point);
}

fn get_largest_i32(i32_list: &[i32]) -> &i32 {
    let mut largest = &i32_list[0];
    for num in i32_list {
        if num > largest {
            largest = num;
        }
    }
    return largest;
}

fn get_largest_char(char_list: &[char]) -> &char {
    let mut largest = &char_list[0];
    for char in char_list {
        if char > largest {
            largest = char;
        }
    }
    return largest;
}

fn get_largest_generic<T: std::cmp::PartialOrd>(generic_list: &[T]) -> &T { // PartialOrd trait required in order to use inequalities
    let mut largest = &generic_list[0];
    for generic in generic_list {
        if generic > largest {
            largest = generic;
        }
    }
    return largest;
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        // get the value of the x attribute of Point
        return &self.x;
    }
}

impl Point<f32> { // only Points of type Point<f32> will have this method
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct MultivalPoint<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}
