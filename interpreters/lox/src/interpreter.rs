use crate::stmt;
use crate::environment;
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

    pub fn interpret(&mut self, stmts: Vec<stmt::Stmt>) -> Result<(), String> {
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
                    println!("\"{}\"", value.to_string());
                }
                stmt::Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(
                        Rc::get_mut(&mut self.environment)
                        .expect("could not get mutable reference to environemnt"),
                        )?;

                    Rc::get_mut(&mut self.environment)
                        .expect("could not get mutable reference to environemnt")
                        .define(name.lexeme, value);
                }
                stmt::Stmt::Block { statements } => {
                    let mut new_environment = environment::Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(new_environment);
                    let block_result = self.interpret(statements);
                    self.environment = old_environment;

                    block_result?;
                }
            };
        }

        return Ok(());
    }
}

