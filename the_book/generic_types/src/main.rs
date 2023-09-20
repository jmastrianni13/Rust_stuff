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

    let point_int = PointMixin{x:5, y:10};
    let point_char = PointMixin{x:'x', y:'y'};
    let point_mixin = point_int.mixin(point_char);
    println!("point_mixin == {:?}", point_mixin);
    
    let option_i32 = Option_i32::Some(5);
    println!("option_i32 == {:?}", option_i32);
    let option_f64 = Option_f64::Some(5.0);
    println!("option_f64 == {:?}", option_f64);
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

#[derive(Debug)]
struct PointMixin<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> PointMixin<X1, Y1> {
    fn mixin<X2, Y2>(self, other: PointMixin<X2, Y2>) -> PointMixin<X1, Y2> {
        return PointMixin{
            x:self.x,
            y:other.y
        };
    }
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

#[derive(Debug)]
enum Option_i32 {
    Some(i32),
    None,
}

#[derive(Debug)]
enum Option_f64 {
    Some(f64),
    None,
}

