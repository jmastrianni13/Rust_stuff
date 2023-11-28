use crate::environment;
use crate::scanner;
use std::cell::RefCell;
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
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone)]
pub enum Expr {
    Assign {
        name: scanner::Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: scanner::Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: scanner::Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Logical {
        left: Box<Expr>,
        operator: scanner::Token,
        right: Box<Expr>,
    },
    Unary {
        operator: scanner::Token,
        right: Box<Expr>,
    },
    Variable {
        name: scanner::Token,
    },
}

impl Expr {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Expr::Assign { name, value } => format!("({name:?} = {})", value.to_string()),
            Expr::Binary {
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
                callee,
                paren: _paren,
                arguments,
            } => format!("({} {:?})", (*callee).to_string(), arguments),
            Expr::Grouping { expression } => format!("(group {})", (*expression).to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Logical {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.to_string(),
                left.to_string(),
                right.to_string()
            ),
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                return format!("({} {})", operator_str, right_str);
            }
            Expr::Variable { name } => format!("(var {})", name.lexeme),
        }
    }

    pub fn evaluate(
        &self,
        env: Rc<RefCell<environment::Environment>>,
    ) -> Result<LiteralValue, String> {
        match self {
            Expr::Assign { name, value } => {
                let new_value = (*value).evaluate(env.clone())?;
                let assign_success = env.borrow_mut().assign(&name.lexeme, new_value.clone());
                if assign_success {
                    return Ok(new_value);
                } else {
                    return Err(format!("variable '{}' has not been declared", name.lexeme));
                }
            }
            Expr::Variable { name } => match env.borrow().get(&name.lexeme) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("variable '{}' has not been declared", name.lexeme)),
            },
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                let callable = (*callee).evaluate(env.clone())?;
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
                            let val = arg.evaluate(env.clone())?;
                            arg_vals.push(val);
                        }
                        return Ok(fun(&arg_vals));
                    }
                    other => Err(format!("{} is not a callable", other.to_type())),
                }
            }
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Logical {
                left,
                operator,
                right,
            } => match operator.token_type {
                scanner::TokenType::Or => {
                    let lhs_value = left.evaluate(env.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == LiteralValue::True {
                        return Ok(lhs_value);
                    } else {
                        return right.evaluate(env.clone());
                    }
                }
                scanner::TokenType::And => {
                    let lhs_value = left.evaluate(env.clone())?;
                    let lhs_true = lhs_value.is_truthy();
                    if lhs_true == LiteralValue::False {
                        return Ok(lhs_true);
                    } else {
                        return right.evaluate(env.clone());
                    }
                }
                ttype => Err(format!("Invalid token in logical expression: {}", ttype)),
            },
            Expr::Grouping { expression } => expression.evaluate(env.clone()),
            Expr::Unary { operator, right } => {
                let right = right.evaluate(env.clone())?;

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
                left,
                operator,
                right,
            } => {
                let left = left.evaluate(env.clone())?;
                let right = right.evaluate(env.clone())?;

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

    #[test]
    fn pretty_print_ast() {
        let minus_token = scanner::Token {
            token_type: scanner::TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };
        let onetwothree = Box::from(Expr::Literal {
            value: LiteralValue::Number(123.0),
        });
        let group = Expr::Grouping {
            expression: Box::from(Expr::Literal {
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
            left: Box::from(Expr::Unary {
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
