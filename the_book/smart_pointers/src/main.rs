fn main() {
    let b = get_box(5);
    println!("b = {b}");

    let lisp_list = get_lisp_list();
    println!("lisp_list = {:?}", lisp_list);

    demo_ref();
}

fn get_box(x: i32) -> Box<i32> {
    let b = Box::new(x);
    return b;
}

fn get_lisp_list() -> LispList {
    let lisp_list = LispList::Cons(1, Box::new(LispList::Cons(2, Box::new(LispList::Cons(3, Box::new(LispList::Nil))))));
    return lisp_list;
}

#[derive(Debug)]
enum LispList {
    Cons(i32, Box<LispList>),
    Nil,
}

fn demo_ref() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

