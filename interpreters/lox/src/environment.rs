use std::collections::HashMap;
use crate::expr;

pub struct Environment {
    values: HashMap<String, expr::LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            values: HashMap::new()
        };
    }

    pub fn define(&mut self, name: String, value: expr::LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&expr::LiteralValue> {
        return self.values.get(name);
    }
}

