use std::ops::Deref;
use std::cell::RefCell;
use std::rc::Rc;

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

    demo_rc();

    demo_refcell_rc();

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

#[derive(Debug)]
enum RcLispList {
    Cons(i32, Rc<RcLispList>),
    Nil,
}

#[derive(Debug)]
enum RefCellRcLispList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellRcLispList>),
    Nil,
}

fn demo_rc() {
    // commented out lines do not compile because a is moved when b is created
    // let a = LispList::Cons(5, Box::new(LispList::Cons(10, Box::new(LispList::Nil))));
    let a = Rc::new(RcLispList::Cons(5, Rc::new(RcLispList::Cons(10, Rc::new(RcLispList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = LispList::Cons(3, Box::new(a));
    let b = RcLispList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    // let c = LispList::Cons(4, Box::new(a));
    let c = RcLispList::Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));

    {
        let d = RcLispList::Cons(3, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }

    println!("count after destroying d = {}", Rc::strong_count(&a));

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

}

fn demo_refcell_rc() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(RefCellRcLispList::Cons(
            Rc::clone(&value),
            Rc::new(RefCellRcLispList::Cons(
                    Rc::new(RefCell::new(10)),
                    Rc::new(RefCellRcLispList::Nil)
                    )
                )
            )
        );
    let b = RefCellRcLispList::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCellRcLispList::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

}

