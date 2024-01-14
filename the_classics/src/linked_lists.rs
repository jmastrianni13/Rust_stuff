use std::fmt;

pub fn main() {
    let my_node: Node<i32> = Node {
        data: Some(5),
        next: None,
    };

    println!("{:?}", my_node);

    let mut my_sll: SingleLinkedList<i32> = SingleLinkedList::new();
    my_sll.insert(5);
    my_sll.insert(10);
    my_sll.insert(20);
    my_sll.insert(40);
    print_sll(&my_sll);

    let mut my_sll: SingleLinkedList<char> = SingleLinkedList::new();
    my_sll.insert('e');
    my_sll.insert('d');
    my_sll.insert('c');
    my_sll.insert('b');
    my_sll.insert('a');
    print_sll(&my_sll);
}

struct Node<T> {
    data: Option<T>,
    next: Option<Box<Node<T>>>,
}

impl<T> fmt::Debug for Node<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data.as_ref().unwrap())
            .field("next", &self.next)
            .finish()
    }
}

#[derive(Debug)]
struct SingleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        return Self { head: None };
    }

    pub fn insert(&mut self, data: T) {
        let new_node = Node {
            data: Some(data),
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
}

fn print_sll<T: fmt::Debug>(sll: &SingleLinkedList<T>) {
    let mut curr_node = &sll.head;
    while let Some(node) = curr_node {
        print!("{:?}->", node.data.as_ref().unwrap());
        curr_node = &node.next;
    }
    println!("{:?}", curr_node);
}
