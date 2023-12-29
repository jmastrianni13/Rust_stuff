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
    locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        return Self {
            values: get_globals(),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
        };
    }

    pub fn resolve(&self, locals: HashMap<usize, usize>) {
        for (key, val) in locals.iter() {
            self.locals.borrow_mut().insert(*key, *val);
        }
    }

    pub fn enclose(&self) -> Environment {
        return Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            locals: self.locals.clone(),
            enclosing: Some(Box::new(self.clone())),
        };
    }

    pub fn define(&self, name: String, value: expr::LiteralValue) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str, expr_id: usize) -> Option<expr::LiteralValue> {
        let distance = self.locals.borrow().get(&expr_id).cloned();
        self.get_internal(name, distance)
    }

    fn get_internal(&self, name: &str, distance: Option<usize>) -> Option<expr::LiteralValue> {
        if let None = distance {
            match &self.enclosing {
                None => self.values.borrow().get(name).cloned(),
                Some(env) => env.get_internal(name, distance),
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
                        env.get_internal(name, Some(distance - 1))
                    }
                }
            }
        }
    }

    pub fn assign_global(&self, name: &str, value: expr::LiteralValue) -> bool {
        return self.assign_internal(name, value, None);
    }

    pub fn assign(&self, name: &str, value: expr::LiteralValue, expr_id: usize) -> bool {
        // ! important that this ID matches with the resolver
        let distance = self.locals.borrow().get(&expr_id).cloned();
        self.assign_internal(name, value, distance)
    }

    fn assign_internal(
        &self,
        name: &str,
        value: expr::LiteralValue,
        distance: Option<usize>,
    ) -> bool {
        if let None = distance {
            match &self.enclosing {
                Some(env) => env.assign_internal(name, value, distance),
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
                    Some(env) => env.assign_internal(name, value, Some(distance - 1)),
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
        let _environment = Environment::new(HashMap::new());
    }
}
