use crate::expr;
use crate::stmt;

pub struct Interpreter {
    // Global state
}

impl Interpreter {
    pub fn new() -> Self {
        return Self { };
    }

    pub fn interpret_exp(&mut self, exp: expr::Expr) -> Result<expr::LiteralValue, String> {
        return exp.evaluate();
    }

    pub fn interpret(&mut self, statements: Vec<stmt::Stmt>) -> Result<(), String> {
        for statement in statements {
            match statement {
                stmt::Stmt::Expression { expression } => {
                    expression.evaluate()?;
                }
                stmt::Stmt::Print { expression } => {
                    let value = expression.evaluate()?;
                    println!("{:?}", value);
                }
            };
        }

        return Ok(());
    }
}

