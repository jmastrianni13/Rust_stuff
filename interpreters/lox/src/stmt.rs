use crate::expr;
use crate::scanner;

pub enum Stmt {
    Expression { expression: expr::Expr },
    Print { expression: expr::Expr },
    Var { name: scanner::Token, initializer: expr::Expr },
}

