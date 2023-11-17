use crate::expr;
use crate::stmt;
use crate::environment;

pub struct Interpreter<'a> {
    environment: environment::Environment<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new() -> Self {
        return Self {
            environment: environment::Environment::new(),
        };
    }

    pub fn interpret(&mut self, statements: Vec<stmt::Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                stmt::Stmt::Expression { expression } => {
                    expression.evaluate(&mut self.environment)?;
                }
                stmt::Stmt::Print { expression } => {
                    let value = expression.evaluate(&mut self.environment)?;
                    println!("{:?}", value);
                }
                stmt::Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(&mut self.environment)?;
                    self.environment.define(name.lexeme, value);
                }
            };
        }

        return Ok(());
    }
}

