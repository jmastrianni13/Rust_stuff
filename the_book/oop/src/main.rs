mod blog;

use oop;

fn main() {
    demo_averagedcollection();
    demo_draw();
    demo_blog();
}

fn demo_averagedcollection () {
    let mut ac = oop::AveragedCollection::new();
    ac.add(5);
    println!("average is {}", ac.get_average());
    ac.add(6);
    println!("average is {}", ac.get_average());
    ac.add(10);
    println!("average is {}", ac.get_average());
    ac.remove();
    println!("average is {}", ac.get_average());
    ac.remove();
    println!("average is {}", ac.get_average());
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl oop::Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn demo_draw () {
    let screen = oop::Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(oop::Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

fn demo_blog () {
    let mut post = blog::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
