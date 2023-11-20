use std::collections::HashMap;
use crate::expr;

pub struct Environment {
    values: HashMap<String, expr::LiteralValue>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            values: HashMap::new(),
            enclosing: None, 
        }
    }

    pub fn define(&mut self, name: String, value: expr::LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&expr::LiteralValue> {
        return self.values.get(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_init() {
        let environment = Environment::new();
    }
}

