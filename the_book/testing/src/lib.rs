pub fn greeting(name: &str) -> String {
    return format!("Hello {}!", name);
}

pub fn failed_greeting(_name: &str) -> String {
    return format!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn failed_test_example() {
        let result = failed_greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
}

