use crate::expr;
use crate::scanner;

#[derive(Debug)]
pub enum Stmt {
    Expression {
        expression: expr::Expr,
    },
    Print {
        expression: expr::Expr,
    },
    Var {
        name: scanner::Token,
        initializer: expr::Expr,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
    IfStmt {
        predicate: expr::Expr,
        then: Box<Stmt>,
        els: Option<Box<Stmt>>,
    },
    WhileStmt {
        condition: expr::Expr,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn tostring(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var { name, initializer } => format!("(var {})", name.lexeme),
            Stmt::Block { statements } => format!(
                "(block {})",
                statements
                    .into_iter()
                    .map(|stmt| stmt.tostring())
                    .collect::<String>()
            ),
            Stmt::IfStmt {
                predicate,
                then,
                els,
            } => todo!(),
            Stmt::WhileStmt { condition, body } => todo!(),
        }
    }
}
