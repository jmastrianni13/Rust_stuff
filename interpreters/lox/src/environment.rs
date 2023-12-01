use crate::expr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn get_globals() -> HashMap<String, expr::LiteralValue> {
    let mut env = HashMap::new();
    env.insert(
        "clock".to_string(),
        expr::LiteralValue::Callable {
            name: "clock".to_string(),
            arity: 0,
            fun: Rc::new(clock_impl),
        },
    );

    return env;
}

fn clock_impl(_args: &Vec<expr::LiteralValue>) -> expr::LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("could not get system time")
        .as_millis();

    return expr::LiteralValue::Number(now as f64 / 1000.0);
}

pub struct Environment {
    globals: Rc<HashMap<String, expr::LiteralValue>>,
    values: HashMap<String, expr::LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            globals: Rc::new(get_globals()),
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
            (None, None) => self.globals.get(name).cloned(),
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
