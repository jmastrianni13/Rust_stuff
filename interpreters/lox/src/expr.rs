use crate::environment;
use crate::expr;
use crate::interpreter;
use crate::scanner;
use crate::stmt;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn unwrap_as_f64(literal: Option<scanner::LiteralValue>) -> f64 {
    match literal {
        Some(scanner::LiteralValue::FValue(x)) => x as f64,
        _ => panic!("cloud not unwrap as f64"),
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        _ => panic!("cloud not unwrap as string"),
    }
}

#[derive(Clone)]
pub enum LiteralValue {
    Number(f64),
    StringLit(String),
    True,
    False,
    Nil,
    Callable {
        name: String,
        arity: usize,
        fun: Rc<dyn Fn(&Vec<LiteralValue>) -> LiteralValue>,
    },
    LoxClass {
        name: String,
        // methods: Vec<(String, LiteralValue)>, // TODO Could also add static fields?
    },
    LoxInstance {
        class: Box<LiteralValue>,
        fields: Rc<RefCell<Vec<(String, LiteralValue)>>>,
    },
}

impl std::fmt::Debug for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for LiteralValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LiteralValue::Number(x), LiteralValue::Number(y)) => x == y,
            (LiteralValue::StringLit(s1), LiteralValue::StringLit(s2)) => s1 == s2,
            (LiteralValue::True, LiteralValue::True) => true,
            (LiteralValue::False, LiteralValue::False) => true,
            (
                LiteralValue::Callable {
                    name: name_1,
                    arity: arity_1,
                    fun: _,
                },
                LiteralValue::Callable {
                    name: name_2,
                    arity: arity_2,
                    fun: _,
                },
            ) => name_1 == name_2 && arity_1 == arity_2,
            _ => false,
        }
    }
}

macro_rules! class_name {
    ($class:expr) => {{
        if let LiteralValue::LoxClass { name } = &**$class {
            name
        } else {
            panic!("unreachable")
        }
    }};
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringLit(x) => format!("\"{}\"", x),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
            LiteralValue::Callable {
                name,
                arity,
                fun: _,
            } => format!("{name}/{arity}"),
            LiteralValue::LoxClass { name } => format!("class '{name}'"),
            LiteralValue::LoxInstance { class, fields: _ } => {
                format!("instance of '{}'", class_name!(class))
            }
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            LiteralValue::Number(_) => "Number",
            LiteralValue::StringLit(_) => "String",
            LiteralValue::True => "Boolean",
            LiteralValue::False => "Boolean",
            LiteralValue::Nil => "Nil",
            LiteralValue::Callable {
                name: _,
                arity: _,
                fun: _,
            } => "Callable",
            LiteralValue::LoxClass { name: _ } => "Class",
            LiteralValue::LoxInstance { class, fields: _ } => &class_name!(class),
        }
    }

    pub fn from_token(token: scanner::Token) -> Self {
        match token.token_type {
            scanner::TokenType::NumberLit => Self::Number(unwrap_as_f64(token.literal)),
            scanner::TokenType::StringLit => Self::StringLit(unwrap_as_string(token.literal)),
            scanner::TokenType::False => Self::False,
            scanner::TokenType::True => Self::True,
            scanner::TokenType::Nil => Self::Nil,
            _ => panic!(
                "cannot create LiteralValue from uknown token_type {:?}",
                token
            ),
        }
    }

    pub fn from_bool(b: bool) -> Self {
        if b {
            return LiteralValue::True;
        } else {
            return LiteralValue::False;
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            LiteralValue::Number(x) => {
                if *x == 0.0 as f64 {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                }
            }
            LiteralValue::StringLit(s) => {
                if s.len() == 0 {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                }
            }
            LiteralValue::True => LiteralValue::False,
            LiteralValue::False => LiteralValue::True,
            LiteralValue::Nil => LiteralValue::True,
            LiteralValue::Callable {
                name: _,
                arity: _,
                fun: _,
            } => {
                panic!("cannot use callable as a falsy value")
            }
            LiteralValue::LoxClass { name: _ } => panic!("cannot use class as a falsy value"),
            LiteralValue::LoxInstance {
                class: _,
                fields: _,
            } => {
                panic!("cannot use class instance as a falsy value")
            }
        }
    }

    pub fn is_truthy(&self) -> LiteralValue {
        match self {
            LiteralValue::Number(x) => {
                if *x == 0.0 as f64 {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            }
            LiteralValue::StringLit(s) => {
                if s.len() == 0 {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            }
            LiteralValue::True => LiteralValue::True,
            LiteralValue::False => LiteralValue::False,
            LiteralValue::Nil => LiteralValue::False,
            LiteralValue::Callable {
                name: _,
                arity: _,
                fun: _,
            } => {
                panic!("cannot use callable as a truthy value")
            }
            LiteralValue::LoxClass { name: _ } => panic!("cannot use class as a truthy value"),
            LiteralValue::LoxInstance {
                class: _,
                fields: _,
            } => {
                panic!("cannot use class instance as a truthy value")
            }
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl core::hash::Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state);
    }
}

impl std::cmp::PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        let ptr_self = std::ptr::addr_of!(self);
        let ptr_other = std::ptr::addr_of!(other);

        return ptr_self == ptr_other;
    }
}

impl std::cmp::Eq for Expr {}

#[derive(Clone)]
pub enum Expr {
    AnonFunction {
        id: usize,
        paren: scanner::Token,
        arguments: Vec<scanner::Token>,
        body: Vec<Box<stmt::Stmt>>,
    },
    Assign {
        id: usize,
        name: scanner::Token,
        value: Box<Expr>,
    },
    Binary {
        id: usize,
        left: Box<Expr>,
        operator: scanner::Token,
        right: Box<Expr>,
    },
    Call {
        id: usize,
        callee: Box<Expr>,
        paren: scanner::Token,
        arguments: Vec<Expr>,
    },
    Get {
        id: usize,
        object: Box<Expr>,
        name: scanner::Token,
    },
    Grouping {
        id: usize,
        expression: Box<Expr>,
    },
    Literal {
        id: usize,
        value: LiteralValue,
    },
    Logical {
        id: usize,
        left: Box<Expr>,
        operator: scanner::Token,
        right: Box<Expr>,
    },
    Set {
        id: usize,
        object: Box<Expr>,
        name: scanner::Token,
        value: Box<Expr>,
    },
    Unary {
        id: usize,
        operator: scanner::Token,
        right: Box<Expr>,
    },
    Variable {
        id: usize,
        name: scanner::Token,
    },
}

impl Expr {
    pub fn get_id(&self) -> usize {
        match self {
            Expr::AnonFunction {
                id,
                paren: _,
                arguments: _,
                body: _,
            } => *id,
            Expr::Assign {
                id,
                name: _,
                value: _,
            } => *id,
            Expr::Binary {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expr::Call {
                id,
                callee: _,
                paren: _,
                arguments: _,
            } => *id,
            Expr::Get {
                id,
                object: _,
                name: _,
            } => *id,
            Expr::Grouping { id, expression: _ } => *id,
            Expr::Literal { id, value: _ } => *id,
            Expr::Logical {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expr::Set {
                id,
                object: _,
                name: _,
                value: _,
            } => *id,
            Expr::Unary {
                id,
                operator: _,
                right: _,
            } => *id,
            Expr::Variable { id, name: _ } => *id,
        }
    }

    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Expr::AnonFunction {
                id: _,
                paren: _,
                arguments,
                body: _,
            } => format!("anon/{}", arguments.len()),
            Expr::Assign { id: _, name, value } => format!("({name:?} = {})", value.to_string()),
            Expr::Binary {
                id: _,
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Call {
                id: _,
                callee,
                paren: _paren,
                arguments,
            } => format!("({} {:?})", (*callee).to_string(), arguments),
            Expr::Get {
                id: _,
                object,
                name,
            } => format!("(get {} {})", object.to_string(), name.lexeme),
            Expr::Grouping { id: _, expression } => {
                format!("(group {})", (*expression).to_string())
            }
            Expr::Literal { id: _, value } => format!("{}", value.to_string()),
            Expr::Logical {
                id: _,
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.to_string(),
                left.to_string(),
                right.to_string()
            ),
            Expr::Set {
                id: _,
                object,
                name,
                value,
            } => format!("(set {} {} to {:?}", object.to_string(), name.lexeme, value),
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                return format!("({} {})", operator_str, right_str);
            }
            Expr::Variable { id: _, name } => format!("(var {})", name.lexeme),
        }
    }

    pub fn evaluate(
        &self,
        env: environment::Environment,
        locals: Rc<RefCell<HashMap<usize, usize>>>,
    ) -> Result<LiteralValue, String> {
        match self {
            Expr::AnonFunction {
                id: _,
                paren,
                arguments,
                body,
            } => {
                let arity = arguments.len();
                let locals = locals.clone();
                let arguments: Vec<scanner::Token> =
                    arguments.iter().map(|t| (*t).clone()).collect();
                let body: Vec<Box<stmt::Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                let paren = paren.clone();
                let fun_impl = move |args: &Vec<LiteralValue>| {
                    let mut anon_int =
                        interpreter::Interpreter::for_anon(env.clone(), locals.clone());
                    for (i, arg) in args.iter().enumerate() {
                        anon_int
                            .environment
                            .define(arguments[i].lexeme.clone(), (*arg).clone());
                    }

                    for i in 0..(body.len()) {
                        anon_int.interpret(vec![&body[i]]).expect(&format!(
                            "evaluating failed inside anon function at line {}",
                            paren.line_number
                        ));

                        if let Some(value) = anon_int.specials.borrow().get("return") {
                            return value.clone();
                        }
                    }

                    return expr::LiteralValue::Nil;
                };

                return Ok(LiteralValue::Callable {
                    name: "anon_functin".to_string(),
                    arity,
                    fun: Rc::new(fun_impl),
                });
            }
            Expr::Assign { id: _, name, value } => {
                let distance = locals.borrow().get(&self.get_id()).cloned();
                let new_value = (*value).evaluate(env.clone(), locals.clone())?;
                let assign_success = env.assign(&name.lexeme, new_value.clone(), distance);
                if assign_success {
                    return Ok(new_value);
                } else {
                    return Err(format!("variable '{}' has not been declared", name.lexeme));
                }
            }
            Expr::Variable { id: _, name } => {
                let distance = locals.borrow().get(&self.get_id()).cloned();
                match env.get(&name.lexeme, distance) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("variable '{}' has not been declared", name.lexeme)),
                }
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                let callable: LiteralValue = (*callee).evaluate(env.clone(), locals.clone())?;
                match callable {
                    LiteralValue::Callable { name, arity, fun } => {
                        if arguments.len() != arity {
                            return Err(format!(
                                "callable {} expected {} arguments but got {}",
                                name,
                                arity,
                                arguments.len()
                            ));
                        }
                        let mut arg_vals = vec![];
                        for arg in arguments {
                            let val = arg.evaluate(env.clone(), locals.clone())?;
                            arg_vals.push(val);
                        }
                        return Ok(fun(&arg_vals));
                    }
                    LiteralValue::LoxClass { name: _ } => {
                        if arguments.len() != 0 {
                            return Err(
                                "can only call the constructor with zero arguments".to_string()
                            );
                        }
                        return Ok(LiteralValue::LoxInstance {
                            class: Box::new(callable.clone()),
                            fields: Rc::new(RefCell::new(vec![])),
                        });
                    }
                    other => Err(format!("{} is not a callable", other.to_type())),
                }
            }
            Expr::Literal { id: _, value } => Ok((*value).clone()),
            Expr::Logical {
                id: _,
                left,
                operator,
                right,
            } => match operator.token_type {
                scanner::TokenType::Or => {
                    let lhs_value = left.evaluate(env.clone(), locals.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == LiteralValue::True {
                        return Ok(lhs_value);
                    } else {
                        return right.evaluate(env.clone(), locals.clone());
                    }
                }
                scanner::TokenType::And => {
                    let lhs_value = left.evaluate(env.clone(), locals.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == LiteralValue::False {
                        return Ok(lhs_true);
                    } else {
                        return right.evaluate(env.clone(), locals.clone());
                    }
                }
                ttype => Err(format!("Invalid token in logical expression: {}", ttype)),
            },
            Expr::Get {
                id: _,
                object,
                name,
            } => {
                let obj_value = object.evaluate(env.clone(), locals.clone())?;
                // obj_value should be a LoxInstance
                if let LiteralValue::LoxInstance { class: _, fields } = obj_value {
                    for (field_name, value) in (*fields.borrow()).iter() {
                        if field_name == &name.lexeme {
                            return Ok(value.clone());
                        }
                    }
                    Err(format!("no field named {} on this instance", name.lexeme))
                } else {
                    Err(format!(
                        "cannot access property on type {}",
                        obj_value.to_type()
                    ))
                }
            }
            Expr::Grouping { id: _, expression } => {
                expression.evaluate(env.clone(), locals.clone())
            }
            Expr::Set {
                id: _,
                object,
                name,
                value,
            } => {
                let obj_value = object.evaluate(env.clone(), locals.clone())?;
                if let LiteralValue::LoxInstance { class: _, fields } = obj_value {
                    let value = value.evaluate(env.clone(), locals.clone())?;

                    let mut idx = 0;
                    let mut found = false;
                    for i in 0..(*fields.borrow()).len() {
                        let field_name = &(*fields.borrow())[i].0;
                        if field_name == &name.lexeme {
                            //fields[i].1 = value.clone();
                            idx = i;
                            found = true;
                            break;
                        }
                    }

                    if found {
                        (*fields.borrow_mut())[idx].1 = value.clone();
                    } else {
                        (*fields.borrow_mut()).push((name.lexeme.clone(), value));
                    }

                    return Ok(expr::LiteralValue::Nil);
                } else {
                    Err(format!(
                        "cannot set property on type {}",
                        obj_value.to_type()
                    ))
                }
            }
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let right = right.evaluate(env.clone(), locals.clone())?;

                match (&right, operator.token_type) {
                    (LiteralValue::Number(x), scanner::TokenType::Minus) => {
                        Ok(LiteralValue::Number(-x))
                    }
                    (_, scanner::TokenType::Minus) => Err(format!(
                        "minus operation not supported for {}",
                        right.to_type()
                    )),
                    (any, scanner::TokenType::Bang) => Ok(any.is_falsy()),
                    (_, toktype) => Err(format!("{} is not a valid unary operator", toktype)),
                }
            }
            Expr::Binary {
                id: _,
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(env.clone(), locals.clone())?;
                let right = right.evaluate(env.clone(), locals.clone())?;

                match (&left, operator.token_type, &right) {
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Star,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::Number(x * y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Slash,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::Number(x / y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Plus,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::Number(x + y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Minus,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::Number(x - y)),

                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Greater,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::from_bool(x > y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::GreaterEqual,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::from_bool(x >= y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Less,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::from_bool(x < y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::LessEqual,
                        LiteralValue::Number(y),
                    ) => Ok(LiteralValue::from_bool(x <= y)),

                    (LiteralValue::StringLit(_), op, LiteralValue::Number(_)) => Err(format!(
                        "binary operation {} not supported for inconsistent types",
                        op
                    )),
                    (LiteralValue::Number(_), op, LiteralValue::StringLit(_)) => Err(format!(
                        "binary operation {} not supported for inconsistent types",
                        op
                    )),

                    (
                        LiteralValue::StringLit(s1),
                        scanner::TokenType::Plus,
                        LiteralValue::StringLit(s2),
                    ) => Ok(LiteralValue::StringLit(format!("{}{}", s1, s2))),
                    (
                        LiteralValue::StringLit(s1),
                        scanner::TokenType::Less,
                        LiteralValue::StringLit(s2),
                    ) => Ok(LiteralValue::from_bool(s1 < s2)),
                    (
                        LiteralValue::StringLit(s1),
                        scanner::TokenType::LessEqual,
                        LiteralValue::StringLit(s2),
                    ) => Ok(LiteralValue::from_bool(s1 <= s2)),

                    (x, scanner::TokenType::BangEqual, y) => Ok(LiteralValue::from_bool(x != y)),
                    (x, scanner::TokenType::EqualEqual, y) => Ok(LiteralValue::from_bool(x == y)),

                    (x, toktype, y) => Err(format!(
                        "binary operator {} not implemented for operands {:?} and {:?}",
                        toktype, x, y
                    )),
                }
            }
        }
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn expr_is_hashable() {
        let mut map = HashMap::new();
        let minus_token = scanner::Token {
            token_type: scanner::TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };
        let onetwothree = Box::from(Expr::Literal {
            id: 0,
            value: LiteralValue::Number(123.0),
        });
        let group = Expr::Grouping {
            id: 1,
            expression: Box::from(Expr::Literal {
                id: 2,
                value: LiteralValue::Number(45.67),
            }),
        };
        let multi = scanner::Token {
            token_type: scanner::TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };

        let exp = Expr::Binary {
            id: 4,
            left: Box::from(Expr::Unary {
                id: 5,
                operator: minus_token,
                right: Box::from(onetwothree),
            }),
            operator: multi,
            right: Box::from(group),
        };

        let exp = std::rc::Rc::new(exp);
        map.insert(exp.clone(), 2);
        match map.get(&exp) {
            Some(_) => (),
            None => panic!("unable to get value from hashmap"),
        }

        let minus_token = scanner::Token {
            token_type: scanner::TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };
        let onetwothree = Box::from(Expr::Literal {
            id: 6,
            value: LiteralValue::Number(123.0),
        });
        let group = Expr::Grouping {
            id: 7,
            expression: Box::from(Expr::Literal {
                id: 8,
                value: LiteralValue::Number(45.67),
            }),
        };
        let multi = scanner::Token {
            token_type: scanner::TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };

        let exp = Expr::Binary {
            id: 9,
            left: Box::from(Expr::Unary {
                id: 10,
                operator: minus_token,
                right: Box::from(onetwothree),
            }),
            operator: multi,
            right: Box::from(group),
        };

        let exp = std::rc::Rc::new(exp);
        match map.get(&exp) {
            None => (),
            Some(_) => panic!("incorrectly able to get value from hashmap"),
        }
    }

    #[test]
    fn pretty_print_ast() {
        let minus_token = scanner::Token {
            token_type: scanner::TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };
        let onetwothree = Box::from(Expr::Literal {
            id: 1,
            value: LiteralValue::Number(123.0),
        });
        let group = Expr::Grouping {
            id: 2,
            expression: Box::from(Expr::Literal {
                id: 3,
                value: LiteralValue::Number(45.67),
            }),
        };
        let multi = scanner::Token {
            token_type: scanner::TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            line_number: 0,
        };

        let ast = Expr::Binary {
            id: 4,
            left: Box::from(Expr::Unary {
                id: 5,
                operator: minus_token,
                right: Box::from(onetwothree),
            }),
            operator: multi,
            right: Box::from(group),
        };

        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}
