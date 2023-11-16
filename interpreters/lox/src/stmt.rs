use crate::expr;
use crate::scanner;

#[derive(Debug)]
pub enum Stmt {
    Expression { expression: expr::Expr },
    Print { expression: expr::Expr },
    Var { name: scanner::Token, initializer: expr::Expr },
}

