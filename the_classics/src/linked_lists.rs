pub fn main() {
    let my_node: Node<i32> = Node {
        data: Some(5),
        next: None,
    };

    println!("{:?}", my_node);

    let my_sll: SingleLinkedList<i32> = SingleLinkedList::new();
    println!("{:?}", my_sll);
}

#[derive(Debug)]
struct Node<T> {
    data: Option<T>,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct SingleLinkedList<T> {
    head: Node<T>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        let head: Node<T> = Node {
            data: None,
            next: None,
        };
        return Self { head };
    }
}
