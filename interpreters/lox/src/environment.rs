use crate::expr;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn get_globals() -> Rc<RefCell<HashMap<String, expr::LiteralValue>>> {
    let mut env = HashMap::new();
    env.insert(
        "clock".to_string(),
        expr::LiteralValue::Callable {
            name: "clock".to_string(),
            arity: 0,
            fun: Rc::new(clock_impl),
        },
    );

    return Rc::new(RefCell::new(env));
}

fn clock_impl(_args: &Vec<expr::LiteralValue>) -> expr::LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("could not get system time")
        .as_millis();

    return expr::LiteralValue::Number(now as f64 / 1000.0);
}

#[derive(Clone)]
pub struct Environment {
    values: Rc<RefCell<HashMap<String, expr::LiteralValue>>>,
    // locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            values: get_globals(),
            enclosing: None,
        };
    }

    pub fn define(&self, name: String, value: expr::LiteralValue) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str, distance: Option<usize>) -> Option<expr::LiteralValue> {
        if let None = distance {
            match &self.enclosing {
                None => self.values.borrow().get(name).cloned(),
                Some(env) => env.get(name, distance),
            }
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.borrow().get(name).cloned()
            } else {
                match &self.enclosing {
                    None => panic!("tried to resolve a variable that was defined deeper than the current environment depth"),
                    Some(env) => {
                        assert!(distance > 0);
                        env.get(name, Some(distance - 1))
                    }
                }
            }
        }
    }

    pub fn assign(&self, name: &str, value: expr::LiteralValue, distance: Option<usize>) -> bool {
        if let None = distance {
            match &self.enclosing {
                Some(env) => env.assign(name, value, distance),
                None => match self.values.borrow_mut().insert(name.to_string(), value) {
                    Some(_) => true,
                    None => false,
                },
            }
        } else {
            let distance = distance.unwrap();
            if distance == 0 {
                self.values.borrow_mut().insert(name.to_string(), value);
                return true;
            } else {
                match &self.enclosing {
                    None => panic!("tried to define a variable in a too deep level"),
                    Some(env) => env.assign(name, value, Some(distance - 1)),
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
