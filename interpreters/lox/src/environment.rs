use crate::expr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Environment {
    values: HashMap<String, expr::LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            values: HashMap::new(),
            enclosing: None,
        };
    }

    pub fn define(&mut self, name: String, value: expr::LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn define_top_level(&mut self, name: String, value: expr::LiteralValue) {
        match &self.enclosing {
            None => self.define(name, value),
            Some(env) => env.borrow_mut().define_top_level(name, value),
        }
    }

    pub fn get(&self, name: &str) -> Option<expr::LiteralValue> {
        let value = self.values.get(name);

        match (value, &self.enclosing) {
            (Some(val), _) => Some(val.clone()),
            (None, Some(env)) => env.borrow().get(name),
            (None, None) => None,
        }
    }

    pub fn assign(&mut self, name: &str, value: expr::LiteralValue) -> bool {
        let old_value = self.values.get(name);

        match (old_value, &mut self.enclosing) {
            (Some(_), _) => {
                self.values.insert(name.to_string(), value);
                return true;
            }
            (None, Some(env)) => (env.borrow_mut()).assign(name, value),
            (None, None) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_init() {
        let _environment = Environment::new();
    }
}
