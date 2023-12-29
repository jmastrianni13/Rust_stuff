use crate::environment;
use crate::expr;
use crate::scanner;
use crate::stmt;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
    pub specials: HashMap<String, expr::LiteralValue>,
    pub environment: environment::Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            specials: HashMap::new(),
            environment: environment::Environment::new(HashMap::new()),
        };
    }

    pub fn resolve(&mut self, locals: HashMap<usize, usize>) {
        self.environment.resolve(locals);
    }

    fn for_closure(parent: environment::Environment) -> Self {
        let environment = parent.enclose();

        return Self {
            specials: HashMap::new(),
            environment,
        };
    }

    pub fn for_anon(parent: environment::Environment) -> Self {
        let env = parent.enclose();
        return Self {
            specials: HashMap::new(),
            environment: env,
        };
    }

    pub fn interpret(&mut self, stmts: Vec<&stmt::Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                stmt::Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                }
                stmt::Stmt::Print { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    println!("{}", value.to_string());
                }
                stmt::Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(self.environment.clone())?;

                    self.environment.define(name.lexeme.clone(), value);
                }
                stmt::Stmt::Block { statements } => {
                    let new_environment = self.environment.enclose();

                    let old_environment = self.environment.clone();
                    self.environment = new_environment;
                    let block_result =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;

                    block_result?; // compiler complains if return keyword is used here
                }
                stmt::Stmt::Class { name, methods: _ } => {
                    self.environment
                        .define(name.lexeme.clone(), expr::LiteralValue::Nil);
                    let klass = expr::LiteralValue::LoxClass {
                        name: name.lexeme.clone(),
                    };
                    if !self.environment.assign_global(&name.lexeme, klass) {
                        return Err(format!("class definition failed for {}", name.lexeme));
                    }
                }
                stmt::Stmt::IfStmt {
                    predicate,
                    then,
                    els,
                } => {
                    let truth_value = predicate.evaluate(self.environment.clone())?;
                    if truth_value.is_truthy() == expr::LiteralValue::True {
                        let statements = vec![then.as_ref()];
                        self.interpret(statements)?;
                    } else if let Some(els_stmt) = els {
                        let statements = vec![els_stmt.as_ref()];
                        self.interpret(statements)?;
                    }
                }
                stmt::Stmt::WhileStmt { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;
                    while flag.is_truthy() == expr::LiteralValue::True {
                        let statements = vec![body.as_ref()];
                        self.interpret(statements)?;
                        flag = condition.evaluate(self.environment.clone())?;
                    }
                }
                stmt::Stmt::Function { name, params, body } => {
                    let arity = params.len();

                    let params: Vec<scanner::Token> = params.iter().map(|t| (*t).clone()).collect();

                    let body: Vec<Box<stmt::Stmt>> = body.iter().map(|b| (*b).clone()).collect();

                    let name_clone = name.lexeme.clone();

                    let parent_env = self.environment.clone();
                    // let parent_locals = self.locals.clone();
                    let fun_impl = move |args: &Vec<expr::LiteralValue>| {
                        let mut clos_int = Interpreter::for_closure(parent_env.clone());

                        for (i, arg) in args.iter().enumerate() {
                            clos_int
                                .environment
                                .define(params[i].lexeme.clone(), (*arg).clone());
                        }

                        for i in 0..(body.len()) {
                            clos_int
                                .interpret(vec![body[i].as_ref()])
                                .expect(&format!("evaluating failed inside {}", name_clone));

                            if let Some(value) = clos_int.specials.get("return") {
                                return value.clone();
                            }
                        }

                        return expr::LiteralValue::Nil;
                    };

                    let callable = expr::LiteralValue::Callable {
                        name: name.lexeme.clone(),
                        arity,
                        fun: Rc::new(fun_impl),
                    };

                    self.environment.define(name.lexeme.clone(), callable);
                }
                stmt::Stmt::ReturnStmt { keyword: _, value } => {
                    let eval_val;
                    if let Some(value) = value {
                        eval_val = value.evaluate(self.environment.clone())?;
                    } else {
                        eval_val = expr::LiteralValue::Nil;
                    }
                    self.specials.insert("return".to_string(), eval_val);
                }
            };
        }

        return Ok(());
    }
}
