use crate::expr;
use crate::stmt;
use crate::environment;

pub struct Interpreter {
    environment: environment::Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            environment: environment::Environment::new(),
        };
    }

    pub fn interpret(&mut self, statements: Vec<stmt::Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                stmt::Stmt::Expression { expression } => {
                    expression.evaluate(&self.environment)?;
                }
                stmt::Stmt::Print { expression } => {
                    let value = expression.evaluate(&self.environment)?;
                    println!("{:?}", value);
                }
                stmt::Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(&self.environment)?;
                    self.environment.define(name.lexeme, value);
                }
            };
        }

        return Ok(());
    }
}

