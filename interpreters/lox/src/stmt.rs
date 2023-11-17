use crate::expr;
use crate::scanner;

#[derive(Debug)]
pub enum Stmt<'a> {
    Expression { expression: expr::Expr<'a> },
    Print { expression: expr::Expr<'a> },
    Var { name: scanner::Token<'a>, initializer: expr::Expr<'a> },
}

impl<'a> Stmt<'a> {
    pub fn tostring(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var { name, initializer } => format!("(var {})", name.lexeme),
        }
    }
}
