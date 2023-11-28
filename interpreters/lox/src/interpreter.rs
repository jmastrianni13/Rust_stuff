use crate::environment;
use crate::expr;
use crate::scanner;
use crate::stmt;
use std::cell::RefCell;
use std::rc::Rc;

fn clock_impl(_args: &Vec<expr::LiteralValue>) -> expr::LiteralValue {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("could not get system time")
        .as_millis();

    return expr::LiteralValue::Number(now as f64 / 1000.0);
}

pub struct Interpreter {
    pub specials: Rc<RefCell<environment::Environment>>,
    pub environment: Rc<RefCell<environment::Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = environment::Environment::new();
        env.define(
            "clock".to_string(),
            expr::LiteralValue::Callable {
                name: "clock".to_string(),
                arity: 0,
                fun: Rc::new(clock_impl),
            },
        );
        return Self {
            specials: Rc::new(RefCell::new(environment::Environment::new())),
            environment: Rc::new(RefCell::new(env)),
        };
    }

    fn for_closure(parent: Rc<RefCell<environment::Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(environment::Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        return Self {
            specials: Rc::new(RefCell::new(environment::Environment::new())),
            environment,
        };
    }

    pub fn for_anon(parent: Rc<RefCell<environment::Environment>>) -> Self {
        let mut env = environment::Environment::new();
        env.enclosing = Some(parent);
        return Self {
            specials: Rc::new(RefCell::new(environment::Environment::new())),
            environment: Rc::new(RefCell::new(env)),
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

                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.clone(), value);
                }
                stmt::Stmt::Block { statements } => {
                    let mut new_environment = environment::Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;

                    block_result?; // compiler complains if return keyword is used here
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
                    let fun_impl = move |args: &Vec<expr::LiteralValue>| {
                        let mut clos_int = Interpreter::for_closure(parent_env.clone());

                        for (i, arg) in args.iter().enumerate() {
                            clos_int
                                .environment
                                .borrow_mut()
                                .define(params[i].lexeme.clone(), (*arg).clone());
                        }

                        for i in 0..(body.len()) {
                            clos_int
                                .interpret(vec![body[i].as_ref()])
                                .expect(&format!("evaluating failed inside {}", name_clone));

                            if let Some(value) = clos_int.specials.borrow().get("return") {
                                return value;
                            }
                        }

                        return expr::LiteralValue::Nil;
                    };

                    let callable = expr::LiteralValue::Callable {
                        name: name.lexeme.clone(),
                        arity,
                        fun: Rc::new(fun_impl),
                    };

                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.clone(), callable);
                }
                stmt::Stmt::ReturnStmt { keyword: _, value } => {
                    let eval_val;
                    if let Some(value) = value {
                        eval_val = value.evaluate(self.environment.clone())?;
                    } else {
                        eval_val = expr::LiteralValue::Nil;
                    }
                    self.specials
                        .borrow_mut()
                        .define_top_level("return".to_string(), eval_val);
                }
            };
        }

        return Ok(());
    }
}
