use crate::environment;
use crate::expr;
use crate::stmt;
use std::rc::Rc;

pub struct Interpreter {
    environment: Rc<environment::Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            environment: Rc::new(environment::Environment::new()),
        };
    }

    pub fn interpret(&mut self, stmts: Vec<&stmt::Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                stmt::Stmt::Expression { expression } => {
                    expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("could not get mutable reference to environment"),
                    )?;
                }
                stmt::Stmt::Print { expression } => {
                    let value = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("could not get mutable reference to environment"),
                    )?;
                    println!("{}", value.to_string());
                }
                stmt::Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("could not get mutable reference to environemnt"),
                    )?;

                    Rc::get_mut(&mut self.environment)
                        .expect("could not get mutable reference to environemnt")
                        .define(name.lexeme.clone(), value);
                }
                stmt::Stmt::Block { statements } => {
                    let mut new_environment = environment::Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(new_environment);
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
                    let truth_value = predicate.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("could not get mutable reference to environment"),
                    )?;
                    if truth_value.is_truthy() == expr::LiteralValue::True {
                        let statements = vec![then.as_ref()];
                        self.interpret(statements)?;
                    } else if let Some(els_stmt) = els {
                        let statements = vec![els_stmt.as_ref()];
                        self.interpret(statements)?;
                    }
                }
                stmt::Stmt::WhileStmt { condition, body } => {
                    let mut flag = condition.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("could not get mutable ref to env"),
                    )?;
                    while flag.is_truthy() == expr::LiteralValue::True {
                        let statements = vec![body.as_ref()];
                        self.interpret(statements)?;
                        flag = condition.evaluate(
                            Rc::get_mut(&mut self.environment)
                                .expect("could not get mutable ref to env"),
                        )?;
                    }
                }
            };
        }

        return Ok(());
    }
}
