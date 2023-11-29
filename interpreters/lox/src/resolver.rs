use crate::interpreter;
use crate::stmt;
use std::collections::HashMap;

pub struct Resolver {
    interp: interpreter::Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new() -> Self {
        return Self {
            interp: interpreter::Interpreter::new(),
            scopes: vec![],
        };
    }

    pub fn resolve(&mut self, stm: &stmt::Stmt) {
        match stm {
            stmt::Stmt::Block { statements: _ } => self.resolve_block(stm),
            _ => todo!(),
        }
        todo!();
    }

    fn resolve_many(&mut self, stmts: &Vec<Box<stmt::Stmt>>) {
        for stm in stmts {
            self.resolve(stm.as_ref());
        }
    }

    fn resolve_block(&mut self, stm: &stmt::Stmt) {
        match stm {
            stmt::Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_many(statements);
                self.end_scope();
            }
            _ => panic!("incorrect type"),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().expect("stack underflow in scope");
    }
}
