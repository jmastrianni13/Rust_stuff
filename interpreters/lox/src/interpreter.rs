use crate::expr;

pub struct Interpreter {
    // Global state
}

impl Interpreter {
    pub fn new() -> Self {
        return Self { };
    }

    pub fn interpret(&mut self, exp: expr::Expr) -> Result<expr::LiteralValue, String> {
        return exp.evaluate();
    }
}

