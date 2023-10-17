pub fn add_two(op: i32) -> i32 {
    return op + 2;
}

pub fn add(left: usize, right: usize) -> usize {
    return left + right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }
}
