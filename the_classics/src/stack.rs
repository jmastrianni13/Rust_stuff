pub fn main() {
    let mut stack: Stack<char> = Stack::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');
    println!("{:?}", stack);
}

#[derive(Debug, PartialEq)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        return Stack { data: vec![] };
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    pub fn peek(&self) -> &T {
        return self.data.last().unwrap();
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn pop(&mut self) -> T {
        return self.data.pop().unwrap();
    }

    pub fn size(&self) -> usize {
        return self.data.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_init() {
        let stack: Stack<i32> = Stack { data: vec![] };
        assert_eq!(Stack::new(), stack);
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<i32> = Stack { data: vec![] };
        stack.push(1);
        assert_eq!(stack.data, vec![1]);
        stack.push(2);
        assert_eq!(stack.data, vec![1, 2]);
        stack.push(3);
        assert_eq!(stack.data, vec![1, 2, 3]);
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32> = Stack { data: vec![] };
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let first_pop = stack.pop();
        assert_eq!(first_pop, 3);
        let second_pop = stack.pop();
        assert_eq!(second_pop, 2);
        let third_pop = stack.pop();
        assert_eq!(third_pop, 1);
    }

    #[test]
    fn test_is_empty() {
        let mut stack: Stack<i32> = Stack { data: vec![] };
        assert_eq!(stack.is_empty(), true);
        stack.push(1);
        assert_eq!(stack.is_empty(), false);
        stack.push(2);
        assert_eq!(stack.is_empty(), false);
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<i32> = Stack { data: vec![] };
        stack.push(1);
        stack.push(2);
        stack.push(3);
        let peeked_val = stack.peek();
        assert_eq!(peeked_val, &3);
        assert_eq!(stack.data, vec![1, 2, 3]);
        let _ = stack.pop();
        let peeked_val = stack.peek();
        assert_eq!(peeked_val, &2);
        assert_eq!(stack.data, vec![1, 2]);
    }

    #[test]
    fn test_size() {
        let mut stack: Stack<i32> = Stack { data: vec![] };
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.size(), 3);
    }
}
