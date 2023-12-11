use crate::expr;
use crate::scanner;

#[derive(Debug, Clone)]
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
    Class {
        name: scanner::Token,
        methods: Vec<Box<Stmt>>,
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
    Function {
        name: scanner::Token,
        params: Vec<scanner::Token>,
        body: Vec<Box<Stmt>>,
    },
    ReturnStmt {
        keyword: scanner::Token,
        value: Option<expr::Expr>,
    },
}

impl Stmt {
    #[allow(dead_code)]
    pub fn tostring(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var {
                name,
                initializer: _,
            } => format!("(var {})", name.lexeme),
            Stmt::Block { statements } => format!(
                "(block {})",
                statements
                    .into_iter()
                    .map(|stmt| stmt.tostring())
                    .collect::<String>()
            ),
            Stmt::IfStmt {
                predicate: _,
                then: _,
                els: _,
            } => todo!(),
            Stmt::WhileStmt {
                condition: _,
                body: _,
            } => todo!(),
            Stmt::Function {
                name: _,
                params: _,
                body: _,
            } => todo!(),
            Stmt::ReturnStmt {
                keyword: _,
                value: _,
            } => todo!(),
            _ => todo!(),
        }
    }
}
