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
    my_sll.reverse();
    print_sll(&my_sll);
}

#[derive(PartialEq)]
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

#[derive(Debug, PartialEq)]
struct SingleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialEq> SingleLinkedList<T> {
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

    pub fn contains(&self, val: &T) -> bool {
        let mut node = &self.head;
        while let Some(n) = node {
            if n.data.as_ref() == Some(val) {
                return true;
            }
            node = &n.next;
        }

        return false;
    }

    pub fn reverse(&mut self) {
        let mut prev_node = None;
        let mut curr_node = self.head.take();
        while let Some(mut boxed_node) = curr_node {
            let next_node = boxed_node.next.take();
            boxed_node.next = prev_node.take();
            prev_node = Some(boxed_node);
            curr_node = next_node;
        }

        self.head = prev_node;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sll() {
        let mut my_sll: SingleLinkedList<char> = SingleLinkedList::new();
        my_sll.insert('c');
        my_sll.insert('b');
        my_sll.insert('a');

        assert_eq!(my_sll.contains(&'a'), true);
        assert_eq!(my_sll.contains(&'b'), true);
        assert_eq!(my_sll.contains(&'c'), true);
        assert_eq!(my_sll.contains(&'d'), false);
    }

    #[test]
    fn test_sll_reverse() {
        let mut my_sll: SingleLinkedList<char> = SingleLinkedList::new();
        my_sll.insert('c');
        my_sll.insert('b');
        my_sll.insert('a');

        my_sll.reverse();

        let mut reversed_sll: SingleLinkedList<char> = SingleLinkedList::new();
        reversed_sll.insert('a');
        reversed_sll.insert('b');
        reversed_sll.insert('c');

        assert_eq!(my_sll, reversed_sll);
    }
}
