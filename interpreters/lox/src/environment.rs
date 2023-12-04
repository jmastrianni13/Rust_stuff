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
    globals: Rc<RefCell<HashMap<String, expr::LiteralValue>>>,
    values: HashMap<String, expr::LiteralValue>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            globals: Rc::new(RefCell::new(get_globals())),
            values: HashMap::new(),
            enclosing: None,
        };
    }

    pub fn define(&mut self, name: String, value: expr::LiteralValue, distance: Option<usize>) {
        if let None = distance {
            self.globals.borrow_mut().insert(name, value);
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.insert(name, value);
            } else {
                self.define(name, value, Some(distance - 1));
            }
        }
    }

    pub fn get(&self, name: &str, distance: Option<usize>) -> Option<expr::LiteralValue> {
        if let None = distance {
            return self.globals.borrow().get(name).cloned();
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.get(name).cloned()
            } else {
                match &self.enclosing {
                    None => panic!("tried to resolve a variable that was defined deeper than the current environment depth"),
                    Some(env) => {
                        assert!(distance > 0);
                        env.borrow().get(name, Some(distance - 1))
                    }
                }
            }
        }
    }

    pub fn assign(
        &mut self,
        name: &str,
        value: expr::LiteralValue,
        distance: Option<usize>,
    ) -> bool {
        if let None = distance {
            self.globals.borrow_mut().insert(name.to_string(), value);
            return true;
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.insert(name.to_string(), value);
                return true;
            } else {
                match &self.enclosing {
                    None => panic!("tried to define a variable in a too deep level"),
                    Some(env) => env.borrow_mut().assign(name, value, Some(distance - 1)),
                };
                return true;
            }
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
