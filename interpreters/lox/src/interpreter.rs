use crate::environment;
use crate::expr;
use crate::stmt;
use std::cell::RefCell;
use std::rc::Rc;

fn clock_impl(_args: &Vec<expr::LiteralValue>) -> expr::LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("could not get system time")
        .as_millis();

    return expr::LiteralValue::Number(now as f64 / 1000.0);
}

pub struct Interpreter {
    environment: Rc<RefCell<environment::Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = environment::Environment::new();
        globals.define(
            String::from("clock"),
            expr::LiteralValue::Callable {
                name: "clock".to_string(),
                arity: 0,
                fun: Rc::new(clock_impl),
            },
        );
        return Self {
            environment: Rc::new(RefCell::new(globals)),
        };
    }

    pub fn interpret(&mut self, stmts: Vec<&stmt::Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                stmt::Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                }
                stmt::Stmt::Print { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    println!("{}", value.to_string());
                }
                stmt::Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(self.environment.clone())?;

                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.clone(), value);
                }
                stmt::Stmt::Block { statements } => {
                    let mut new_environment = environment::Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;

                    block_result?; // compiler complains if return keyword is used here
                }
                stmt::Stmt::IfStmt {
                    predicate,
                    then,
                    els,
                } => {
                    let truth_value = predicate.evaluate(self.environment.clone())?;
                    if truth_value.is_truthy() == expr::LiteralValue::True {
                        let statements = vec![then.as_ref()];
                        self.interpret(statements)?;
                    } else if let Some(els_stmt) = els {
                        let statements = vec![els_stmt.as_ref()];
                        self.interpret(statements)?;
                    }
                }
                stmt::Stmt::WhileStmt { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;
                    while flag.is_truthy() == expr::LiteralValue::True {
                        let statements = vec![body.as_ref()];
                        self.interpret(statements)?;
                        flag = condition.evaluate(self.environment.clone())?;
                    }
                }
            };
        }

        return Ok(());
    }
}
