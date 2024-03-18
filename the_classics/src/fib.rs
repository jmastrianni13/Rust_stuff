pub fn main() {
    println!("hello fib");
    let x: i32 = 3;
    let fib_x: i32 = fib_rec(x);
    println!("{}", fib_x);
    let fib_y: i32 = fib_iter(x);
    println!("{}", fib_y);
}

fn fib_rec(n: i32) -> i32 {
    match n {
        0 => return 0,
        1 => return n,
        _ => return fib_rec(n - 2) + fib_rec(n - 1),
    }
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

fn fib_iter(n: i32) -> i32 {
    let mut arg_stack = Vec::<Action<i32, i32>>::new();
    let mut ret_stack = Vec::<i32>::new();

    use Action::*;

    arg_stack.push(Call(n));
    while let Some(action) = arg_stack.pop() {
        match action {
            Call(n) => {
                if n > 1 {
                    arg_stack.push(Handle(n));
                    arg_stack.push(Call(n - 2));
                    arg_stack.push(Call(n - 1));
                } else if n == 0 || n == 1 {
                    ret_stack.push(n);
                }
            }
            Handle(value) => {
                let term_a = ret_stack.pop().unwrap();
                let term_b = ret_stack.pop().unwrap();
                ret_stack.push(term_a + term_b);
            }
        }
    }

    return ret_stack.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_rec() {
        let cases: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        let results: Vec<i32> = vec![0, 1, 1, 2, 3, 5];

        for f in cases {
            assert_eq!(results[f as usize], fib_rec(f));
        }
    }

    fn test_fib_iter() {
        let cases: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        let results: Vec<i32> = vec![0, 1, 1, 2, 3, 5];

        for f in cases {
            assert_eq!(results[f as usize], fib_iter(f));
        }
    }
}
