fn main() {
    let b = get_box(5);
    println!("b = {b}");
}

fn get_box(x: i32) -> Box<i32> {
    let b = Box::new(x);
    return b;
}

fn get_lisp_list(x: u8) -> LispList {
    let lisp_list = LispList::Cons(1, LispList::Cons(2, LispList::Cons(3, LispList::Nil)));
    return lisp_list;
}

enum LispList {
    Cons(i32, LispList),
    Nil,
}

