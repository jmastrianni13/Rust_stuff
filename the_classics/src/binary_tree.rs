//  binary trees
//  example:
//       1
//     /   \
//    2     3
//   / \   / \
//  4   5 6   7

use std::fmt::{Debug, Display};

pub fn main() {
    let tree = generate_tree_nonrec(3);
    print_tree_nonrec(&tree);
    println!("----------------------------");
    print_tree_nonrec(&invert_tree_nonrec(&tree));
}

type NodeRef<T> = Option<Box<Node<T>>>;

#[derive(Default, Debug)]
struct Node<T> {
    value: T,
    left: NodeRef<T>,
    right: NodeRef<T>,
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

#[allow(dead_code)]
fn generate_tree(level: usize, counter: &mut i32) -> NodeRef<i32> {
    if level == 0 {
        return None;
    } else {
        let mut node = Node {
            value: *counter,
            left: None,
            right: None,
        };
        node.value = *counter;
        *counter += 1;
        node.left = generate_tree(level - 1, counter);
        node.right = generate_tree(level - 1, counter);
        return Some(Box::new(node));
    }
}

fn generate_tree_nonrec(level: usize) -> NodeRef<i32> {
    let mut counter = 1;
    let mut arg_stack = Vec::<Action<usize, i32>>::new();
    let mut ret_stack = Vec::<NodeRef<i32>>::new();

    use Action::*;

    arg_stack.push(Call(level));
    while let Some(action) = arg_stack.pop() {
        match action {
            Call(level) => {
                if level > 0 {
                    arg_stack.push(Handle(counter));
                    counter += 1;
                    arg_stack.push(Call(level - 1));
                    arg_stack.push(Call(level - 1));
                } else {
                    ret_stack.push(None);
                }
            }
            Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node { value, left, right })));
            }
        }
    }
    return ret_stack.pop().unwrap();
}

#[allow(dead_code)]
fn print_tree<T: Display>(root: &NodeRef<T>, level: usize) {
    if let Some(node) = root {
        print_tree(&node.right, level + 1);
        for _ in 0..level {
            print!("  ");
        }
        println!("{}", node.value);
        print_tree(&node.left, level + 1);
    }
}

fn print_tree_nonrec<T: Display>(root: &NodeRef<T>) {
    let mut stack = Vec::<Action<(&NodeRef<T>, usize), (&T, usize)>>::new();
    use Action::*;
    stack.push(Call((&root, 0)));
    while let Some(action) = stack.pop() {
        match action {
            Call((root, level)) => {
                if let Some(node) = root {
                    stack.push(Call((&node.left, level + 1)));
                    stack.push(Handle((&node.value, level)));
                    stack.push(Call((&node.right, level + 1)));
                }
            }
            Handle((value, level)) => {
                for _ in 0..level {
                    print!("  ");
                }
                println!("{}", value);
            }
        }
    }
}

#[allow(dead_code)]
fn invert_tree<T: Clone>(root: &NodeRef<T>) -> NodeRef<T> {
    if let Some(node) = root {
        let left = invert_tree(&node.right);
        let right = invert_tree(&node.left);
        return Some(Box::new(Node {
            value: node.value.clone(),
            left,
            right,
        }));
    } else {
        None
    }
}

fn invert_tree_nonrec<T: Clone + Debug>(root: &NodeRef<T>) -> NodeRef<T> {
    let mut arg_stack = Vec::<Action<&NodeRef<T>, &T>>::new();
    let mut ret_stack = Vec::<NodeRef<T>>::new();

    use Action::*;

    arg_stack.push(Call(root));
    while let Some(action) = arg_stack.pop() {
        match action {
            Call(root) => {
                if let Some(node) = root {
                    arg_stack.push(Handle(&node.value));
                    arg_stack.push(Call(&node.right));
                    arg_stack.push(Call(&node.left));
                } else {
                    ret_stack.push(None)
                }
            }
            Handle(value) => {
                let left = ret_stack.pop().unwrap();
                let right = ret_stack.pop().unwrap();
                ret_stack.push(Some(Box::new(Node {
                    value: value.clone(),
                    left,
                    right,
                })));
            }
        }
    }

    return ret_stack.pop().unwrap();
}
