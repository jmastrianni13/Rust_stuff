use std::collections::HashMap;
use crate::expr;

pub struct Environment<'a> {
    values: HashMap<&'a str, expr::LiteralValue<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        return Self {
            values: HashMap::new()
        };
    }

    pub fn define(&mut self, name: &'a str, value: expr::LiteralValue<'a>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&expr::LiteralValue<'a>> {
        return self.values.get(name);
    }
}

