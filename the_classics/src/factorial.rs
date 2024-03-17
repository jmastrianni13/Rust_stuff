pub fn main() {
    println!("5! = {}", fact_rec(5));
    println!("5! = {}", fact_iter(5));
}

fn fact_rec(n: i32) -> i32 {
    match n {
        x if x < 0 => panic!("no negative factorials"),
        0 => 1,
        1 => n,
        _ => n * fact_rec(n - 1),
    }
}

#[derive(Debug)]
enum Action<T, U> {
    Call(T),
    Handle(U),
}

fn fact_iter(n: i32) -> i32 {
    let mut arg_stack = Vec::<Action<i32, i32>>::new();
    let mut ret_stack = Vec::<i32>::new();

    use Action::*;

    arg_stack.push(Call(n));
    while let Some(action) = arg_stack.pop() {
        match action {
            Call(n) => {
                if n > 0 {
                    arg_stack.push(Handle(n));
                    arg_stack.push(Call(n - 1));
                } else if n == 0 {
                    ret_stack.push(1);
                } else {
                    panic!("non-positive integer value provided");
                }
            }
            Handle(value) => {
                let curr_val = ret_stack.pop().unwrap();
                ret_stack.push(value * curr_val);
            }
        }
    }

    return ret_stack.pop().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_rec() {
        let cases: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        let results: Vec<i32> = vec![1, 1, 2, 6, 24, 120];

        for f in cases {
            assert_eq!(results[f as usize], fact_rec(f));
        }
    }

    #[test]
    fn test_fact_iter() {
        let cases: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        let results: Vec<i32> = vec![1, 1, 2, 6, 24, 120];

        for f in cases {
            assert_eq!(results[f as usize], fact_iter(f));
        }
    }
}
