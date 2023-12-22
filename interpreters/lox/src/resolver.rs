use crate::expr;
use crate::interpreter;
use crate::scanner;
use crate::stmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
}

#[allow(dead_code)]
pub struct Resolver {
    pub interp: Rc<RefCell<interpreter::Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl Resolver {
    pub fn new(interp: Rc<RefCell<interpreter::Interpreter>>) -> Self {
        return Self {
            interp,
            scopes: vec![],
            current_function: FunctionType::None,
        };
    }

    pub fn resolve(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        match stm {
            stmt::Stmt::Block { statements: _ } => self.resolve_block(stm)?,
            stmt::Stmt::Var {
                name: _,
                initializer: _,
            } => self.resolve_var(stm)?,
            stmt::Stmt::Class { name, methods: _ } => {
                self.declare(name)?;
                self.define(name);
            }
            stmt::Stmt::Function {
                name: _,
                params: _,
                body: _,
            } => self.resolve_function(stm)?,
            stmt::Stmt::Expression { expression } => self.resolve_expr(expression)?,
            stmt::Stmt::IfStmt {
                predicate: _,
                then: _,
                els: _,
            } => self.resolve_if_stmt(stm)?,
            stmt::Stmt::Print { expression } => self.resolve_expr(expression)?,
            stmt::Stmt::ReturnStmt { keyword: _, value } => {
                if self.current_function == FunctionType::None {
                    return Err("return statement is not allowed outside of a function".to_string());
                }

                if let Some(value) = value {
                    self.resolve_expr(value)?;
                }
            }
            stmt::Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve(body.as_ref())?;
            }
        }

        return Ok(());
    }

    pub fn resolve_many(&mut self, stmts: &Vec<&stmt::Stmt>) -> Result<(), String> {
        for stm in stmts {
            self.resolve(stm)?;
        }
        return Ok(());
    }

    fn resolve_block(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        match stm {
            stmt::Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_many(&statements.iter().map(|b| b.as_ref()).collect())?;
                self.end_scope();
            }
            _ => panic!("incorrect type"),
        }
        return Ok(());
    }

    fn resolve_var(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        if let stmt::Stmt::Var { name, initializer } = stm {
            self.declare(name)?;
            self.resolve_expr(initializer)?;
            self.define(name);
        } else {
            panic!("incorrect type in resolve var");
        }
        return Ok(());
    }

    fn resolve_function(&mut self, stm: &stmt::Stmt) -> Result<(), String> {
        if let stmt::Stmt::Function { name, params, body } = stm {
            self.declare(name)?;
            self.define(name);

            self.resolve_function_helper(
                params,
                &body.iter().map(|b| b.as_ref()).collect(),
                FunctionType::Function,
            )
        } else {
            panic!("incorrect type in resolve function");
        }
    }

    fn resolve_function_helper(
        &mut self,
        params: &Vec<scanner::Token>,
        body: &Vec<&stmt::Stmt>,
        resolving_function: FunctionType,
    ) -> Result<(), String> {
        let enclosing_function = self.current_function;
        self.current_function = resolving_function;
        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve_many(body)?;
        self.end_scope();

        self.current_function = enclosing_function;

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

    fn declare(&mut self, name: &scanner::Token) -> Result<(), String> {
        let size = self.scopes.len();
        if self.scopes.is_empty() {
            return Ok(()); // scopes vec is empty, must be in global scope so do nothing
        }

        if self.scopes[size - 1].contains_key(&name.lexeme.clone()) {
            return Err("a variable with this name is already in scope".to_string());
        }

        self.scopes[size - 1].insert(name.lexeme.clone(), false);

        return Ok(());
    }

    fn define(&mut self, name: &scanner::Token) {
        if self.scopes.is_empty() {
            return; // scopes vec is empty, must be in global scope so do nothing
        }

        let size = self.scopes.len();
        self.scopes[size - 1].insert(name.lexeme.clone(), true);
    }

    fn resolve_expr(&mut self, exp: &expr::Expr) -> Result<(), String> {
        match exp {
            expr::Expr::Variable { id: _, name: _ } => self.resolve_expr_var(exp, exp.get_id()),
            expr::Expr::Assign {
                id: _,
                name: _,
                value: _,
            } => self.resolve_expr_assign(exp, exp.get_id()),
            expr::Expr::Binary {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                return self.resolve_expr(right);
            }
            expr::Expr::Call {
                id: _,
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
            expr::Expr::Get {
                id: _,
                object,
                name: _,
            } => self.resolve_expr(object),
            expr::Expr::Grouping { id: _, expression } => self.resolve_expr(expression),
            expr::Expr::Literal { id: _, value: _ } => Ok(()),
            expr::Expr::Logical {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                return self.resolve_expr(right);
            }
            expr::Expr::Set {
                id: _,
                object,
                name: _,
                value,
            } => {
                self.resolve_expr(object)?;
                self.resolve_expr(value)
            }
            expr::Expr::Unary {
                id: _,
                operator: _,
                right,
            } => self.resolve_expr(right),
            expr::Expr::AnonFunction {
                id: _,
                paren: _,
                arguments,
                body,
            } => self.resolve_function_helper(
                arguments,
                &body.iter().map(|b| b.as_ref()).collect(),
                FunctionType::Function,
            ),
        }
    }

    fn resolve_expr_var(&mut self, exp: &expr::Expr, resolve_id: usize) -> Result<(), String> {
        match exp {
            expr::Expr::Variable { id: _, name } => {
                if !self.scopes.is_empty() {
                    if let Some(false) = self.scopes[self.scopes.len() - 1].get(&name.lexeme) {
                        return Err("cannot read local varaible in its own initializer".to_string());
                    }
                }
                return self.resolve_local(name, resolve_id);
            }
            expr::Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments: _,
            } => match callee.as_ref() {
                expr::Expr::Variable { id: _, name } => self.resolve_local(&name, resolve_id),
                _ => panic!("incorrect type in resolve_expr_var"),
            },
            _ => panic!("incorrect type in resolve_expr_var"),
        }
    }

    fn resolve_local(&mut self, name: &scanner::Token, resolve_id: usize) -> Result<(), String> {
        let size = self.scopes.len();
        if size == 0 {
            return Ok(());
        }

        for i in (0..=(size - 1)).rev() {
            let scope = &self.scopes[i];
            if scope.contains_key(&name.lexeme) {
                self.interp.borrow_mut().resolve(resolve_id, size - 1 - i)?;
                return Ok(());
            }
        }
        return Ok(()); // assume it's global
    }

    fn resolve_expr_assign(&mut self, exp: &expr::Expr, resolve_id: usize) -> Result<(), String> {
        if let expr::Expr::Assign { id: _, name, value } = exp {
            self.resolve_expr(value.as_ref())?;
            self.resolve_local(name, resolve_id)?;
        } else {
            panic!("incorrect type in resolve assign");
        }

        return Ok(());
    }
}
