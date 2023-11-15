use crate::expr;

pub enum Stmt {
    Expression { expression: expr::Expr },
    Print { expression: expr::Expr },
}

