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

impl Resolver {
    #[allow(dead_code)]
    pub fn new() -> Self {
        return Self {
            interp: interpreter::Interpreter::new(),
            scopes: vec![],
        };
    }

    #[allow(dead_code)]
    pub fn resolve(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        match stm {
            stmt::Stmt::Block { statements: _ } => self.resolve_block(stm)?,
            stmt::Stmt::Var {
                name: _,
                initializer: _,
            } => self.resolve_var(stm)?,
            stmt::Stmt::Function { name, params, body } => self.resolve_function(stm)?,
            stmt::Stmt::Expression { expression } => self.resolve_expr(expression)?,
            stmt::Stmt::IfStmt {
                predicate,
                then,
                els,
            } => self.resolve_if_stmt(stm)?,
            stmt::Stmt::Print { expression } => self.resolve_expr(expression)?,
            stmt::Stmt::ReturnStmt {
                keyword: _,
                value: None,
            } => (),
            stmt::Stmt::ReturnStmt {
                keyword: _,
                value: Some(value),
            } => self.resolve_expr(value)?,
            stmt::Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve(body.as_ref())?;
            }
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

    fn resolve_function(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        if let stmt::Stmt::Function { name, params, body } = stm {
            self.declare(name);
            self.define(name);

            return self.resolve_function_helper(params, body);
        } else {
            panic!("incorrect type in resolve function");
        }
    }

    fn resolve_function_helper(
        &mut self,
        params: &Vec<scanner::Token>,
        body: &Vec<Box<stmt::Stmt>>,
    ) -> Result<(), String> {
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_many(body)?;
        self.end_scope();

        return Ok(());
    }

    fn resolve_if_stmt(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        if let stmt::Stmt::IfStmt {
            predicate,
            then,
            els,
        } = stm
        {
            self.resolve_expr(predicate)?;
            self.resolve(then.as_ref())?;
            if let Some(els) = els {
                self.resolve(els.as_ref())?;
            }

            return Ok(());
        } else {
            panic!("incorrect type in resolve if statement");
        }
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
            expr::Expr::Assign { name: _, value: _ } => self.resolve_expr_assign(exp),
            expr::Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                return self.resolve_expr(right);
            }
            expr::Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee.as_ref())?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }

                return Ok(());
            }
            expr::Expr::Grouping { expression } => self.resolve_expr(expression),
            expr::Expr::Literal { value: _ } => Ok(()),
            expr::Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                return self.resolve_expr(right);
            }
            expr::Expr::Unary { operator: _, right } => self.resolve_expr(right),
            expr::Expr::AnonFunction {
                paren: _,
                arguments,
                body,
            } => self.resolve_function_helper(arguments, body),
        }
    }

    fn resolve_expr_var(&mut self, exp: &expr::Expr) -> Result<(), String> {
        if let expr::Expr::Variable { name } = exp {
            let size = self.scopes.len() - 1;
            if !self.scopes.is_empty() && *self.scopes[size].get(&name.lexeme).unwrap() == false {
                return Err("cannot read local varaible in its own initializer".to_string());
            }

            return self.resolve_local(exp, name);
        } else {
            panic!("incorrect type in resolve_expr_var");
        }
    }

    fn resolve_local(&mut self, exp: &expr::Expr, name: &scanner::Token) -> Result<(), String> {
        let size = self.scopes.len();
        for i in (0..=(size - 1)).rev() {
            let scope = self.scopes[i];
            if scope.contains_key(&name.lexeme) {
                self.interp.resolve(exp, size - 1 - i)?;
                return Ok(());
            }
        }
        return Ok(()); // assume it's global
    }

    fn resolve_expr_assign(&mut self, exp: &expr::Expr) -> Result<(), String> {
        if let expr::Expr::Assign { name, value } = exp {
            self.resolve_expr(value.as_ref())?;
            self.resolve_local(exp, name)?;
        } else {
            panic!("incorrect type in resolve assign");
        }

        return Ok(());
    }
}
