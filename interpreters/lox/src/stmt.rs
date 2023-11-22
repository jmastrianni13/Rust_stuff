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
//    ForStmt {
//        var_decl: Option<Box<Stmt>>,
//        expr_stmt: Option<Box<Stmt>>,
//
//        condition: Option<expr::Expr>,
//        incrementer: Option<expr::Expr>,
//
//        body: Box<Stmt>,
//    },
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
        }
    }
}
