use crate::expr;
use crate::scanner;

#[derive(Debug)]
pub enum Stmt {
    Expression { expression: expr::Expr },
    Print { expression: expr::Expr },
    Var { name: scanner::Token, initializer: expr::Expr },
}

impl Stmt {
    pub fn tostring(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var { name, initializer } => format!("(var {})", name.lexeme),
        }
    }
}
