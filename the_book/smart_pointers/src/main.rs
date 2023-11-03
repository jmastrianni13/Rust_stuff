use std::ops::Deref;

fn main() {
    let b = get_box(5);
    println!("b = {b}");

    let lisp_list = get_lisp_list();
    println!("lisp_list = {:?}", lisp_list);

    demo_ref();

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let e = CustomSmartPointer {
        data: String::from("more stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
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
    let y = &x; // reference to x
    let z = Box::new(x); // instance of Box<T> pointing to a copy of x
    let w = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

