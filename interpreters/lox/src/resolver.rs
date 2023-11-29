use crate::expr;
use crate::interpreter;
use crate::scanner;
use crate::stmt;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Resolver {
    interp: interpreter::Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

#[allow(dead_code)]
impl Resolver {
    pub fn new() -> Self {
        return Self {
            interp: interpreter::Interpreter::new(),
            scopes: vec![],
        };
    }

    pub fn resolve(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        match stm {
            stmt::Stmt::Block { statements: _ } => self.resolve_block(stm)?,
            stmt::Stmt::Var {
                name: _,
                initializer: _,
            } => self.resolve_var(stm)?,
            _ => todo!(),
        }
        todo!();
    }

    fn resolve_many(&mut self, stmts: &Vec<Box<stmt::Stmt>>) -> Result<(), String> {
        for stm in stmts {
            self.resolve(stm.as_ref())?;
        }
        return Ok(());
    }

    fn resolve_block(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        match stm {
            stmt::Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_many(statements)?;
                self.end_scope();
            }
            _ => panic!("incorrect type"),
        }
        return Ok(());
    }

    fn resolve_var(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        if let stmt::Stmt::Var { name, initializer } = stm {
            self.declare(name);
            self.resolve_expr(initializer)?;
            self.define(name);
        } else {
            panic!("incorrect type in resolve var");
        }
        return Ok(());
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().expect("stack underflow in scope");
    }

    fn declare(&mut self, name: &scanner::Token) {
        if self.scopes.is_empty() {
            return; // scopes vec is empty, must be in global scope so do nothing
        }

        let size = self.scopes.len() - 1;
        self.scopes[size].insert(name.lexeme.clone(), false);
    }

    fn define(&mut self, name: &scanner::Token) {
        if self.scopes.is_empty() {
            return; // scopes vec is empty, must be in global scope so do nothing
        }

        let size = self.scopes.len() - 1;
        self.scopes[size].insert(name.lexeme.clone(), true);
    }

    fn resolve_expr(&mut self, exp: &expr::Expr) -> Result<(), String> {
        match exp {
            expr::Expr::Variable { name: _ } => self.resolve_expr_var(exp),
            _ => todo!(),
        }
    }

    fn resolve_expr_var(&mut self, exp: &expr::Expr) -> Result<(), String> {
        if let expr::Expr::Variable { name } = exp {
            let size = self.scopes.len() - 1;
            if !self.scopes.is_empty() && *self.scopes[size].get(&name.lexeme).unwrap() == false {
                return Err("cannot read local varaible in its own initializer".to_string());
            }

            return self.resolve_local(exp);
        } else {
            panic!("incorrect type in resolve_expr_var");
        }
    }

    fn resolve_local(&mut self, _exp: &expr::Expr) -> Result<(), String> {
        todo!();
    }
}
