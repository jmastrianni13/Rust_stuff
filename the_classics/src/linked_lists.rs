use std::fmt;

pub fn main() {
    let my_node: Node<i32> = Node {
        data: Some(5),
        next: None,
    };

    println!("{:?}", my_node);

    let mut my_sll: SingleLinkedList<i32> = SingleLinkedList::new();
    println!("{:?}", my_sll);
    my_sll.insert(5);
    my_sll.insert(10);
    println!("{:?}", my_sll);
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

