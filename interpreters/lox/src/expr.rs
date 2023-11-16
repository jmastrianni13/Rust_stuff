use crate::scanner;

fn unwrap_as_f32(literal: Option<scanner::LiteralValue>) -> f32 {
    match literal {
        Some(scanner::LiteralValue::IntValue(x)) => x as f32,
        Some(scanner::LiteralValue::FValue(x)) => x as f32,
        _ => panic!("cloud not unwrap as f32"),
    }
}

fn unwrap_as_string(literal: Option<scanner::LiteralValue>) -> String {
    match literal {
        Some(scanner::LiteralValue::StringValue(s)) => s.clone(),
        Some(scanner::LiteralValue::IdentifierVal(s)) => s.clone(),
        _ => panic!("cloud not unwrap as string"),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    StringLit(String),
    True,
    False,
    Nil,
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringLit(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Nil => "nil".to_string(),
        }
    }

    pub fn to_type(&self) -> &str {
        match self {
            LiteralValue::Number(_) => "Number",
            LiteralValue::StringLit(_) => "String",
            LiteralValue::True => "Boolean",
            LiteralValue::False => "Boolean",
            LiteralValue::Nil => "Nil",
        }
    }

    pub fn from_token(token: scanner::Token) -> Self {
        match token.token_type {
            scanner::TokenType::NumberLit => Self::Number(unwrap_as_f32(token.literal)),
            scanner::TokenType::StringLit => Self::StringLit(unwrap_as_string(token.literal)),
            scanner::TokenType::False => Self::False,
            scanner::TokenType::True => Self::True,
            scanner::TokenType::Nil => Self::Nil,
            _ => panic!("cannot create LiteralValue from uknown token_type {:?}", token)
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
            LiteralValue::Number(x) => if *x == 0.0 as f32 { LiteralValue::True } else { LiteralValue::False },
            LiteralValue::StringLit(s) => if s.len() == 0 { LiteralValue::True } else { LiteralValue::False },
            LiteralValue::True => LiteralValue::False,
            LiteralValue::False => LiteralValue::True,
            LiteralValue::Nil => LiteralValue::True,
        }
    }
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: scanner::Token, right: Box<Expr>},
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: scanner::Token, right: Box<Expr> },
    Var {name: scanner::Token},
}


impl Expr {
    pub fn to_string(&self) -> String{
        match self {
            Expr::Binary {
                left,
                operator,
                right
            } => format!(
                "({} {} {})",
                operator.lexeme,
                left.to_string(),
                right.to_string()
            ),
            Expr::Grouping { expression } => format!("(group {})", (*expression).to_string()),
            Expr::Literal { value } => format!("{}", value.to_string()),
            Expr::Unary { operator, right } =>  {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                return format!("({} {})", operator_str, right_str);
            }
            Expr::Var { name } => format!("(var {})", name.lexeme),
        }
    }

    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        match self {
            Expr::Var { name: name } => todo!(),
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Grouping { expression } => expression.evaluate(),
            Expr::Unary { operator, right } => {
                let right = right.evaluate()?;

                match (&right, operator.token_type) {
                    (LiteralValue::Number(x), scanner::TokenType::Minus) => Ok(LiteralValue::Number(-x)),
                    (_, scanner::TokenType::Minus) => Err(format!("minus operation not supported for {}", right.to_type())),
                    (any, scanner::TokenType::Bang) => Ok(any.is_falsy()),
                    (_, toktype) => Err(format!("{} is not a valid unary operator", toktype)),
                }
            },
            Expr::Binary { left, operator, right } => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;

                match (&left, operator.token_type, &right) {
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Star,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::Number(x * y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Slash,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::Number(x / y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Plus,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::Number(x + y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Minus,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::Number(x - y)),

                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Greater,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::from_bool(x > y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::GreaterEqual,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::from_bool(x >= y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::Less,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::from_bool(x < y)),
                    (
                        LiteralValue::Number(x),
                        scanner::TokenType::LessEqual,
                        LiteralValue::Number(y)
                    ) => Ok(LiteralValue::from_bool(x <= y)),

                    (
                        LiteralValue::StringLit(_),
                        op,
                        LiteralValue::Number(_)
                    ) => Err(format!("binary operation {} not supported for inconsistent types", op)),
                    (
                        LiteralValue::Number(_),
                        op,
                        LiteralValue::StringLit(_)
                    ) => Err(format!("binary operation {} not supported for inconsistent types", op)),

                    (
                        LiteralValue::StringLit(s1),
                        scanner::TokenType::Plus,
                        LiteralValue::StringLit(s2)
                    ) => Ok(LiteralValue::StringLit(format!("{}{}", s1, s2))),
                    (
                        LiteralValue::StringLit(s1),
                        scanner::TokenType::Less,
                        LiteralValue::StringLit(s2)
                    ) => Ok(LiteralValue::from_bool(s1 < s2)),
                    (
                        LiteralValue::StringLit(s1),
                        scanner::TokenType::LessEqual,
                        LiteralValue::StringLit(s2)
                    ) => Ok(LiteralValue::from_bool(s1 <= s2)),

                    (
                        x,
                        scanner::TokenType::BangEqual,
                        y
                    ) => Ok(LiteralValue::from_bool(x != y)),
                    (
                        x,
                        scanner::TokenType::EqualEqual,
                        y
                    ) => Ok(LiteralValue::from_bool(x == y)),

                    (
                        x,
                        toktype,
                        y
                    ) => Err(format!("binary operator {} not implemented for operands {:?} and {:?}", toktype, x, y)),
                }
            }
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_print_ast () {
        let minus_token = scanner::Token {
            token_type: scanner::TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            line_number: 0,
        };
        let onetwothree = Box::from(Expr::Literal {
            value: LiteralValue::Number(123.0)
        });
        let group = Expr::Grouping {
            expression: Box::from(Expr::Literal {
                value: LiteralValue::Number(45.67)
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
                right: Box::from(onetwothree)
            }),
            operator: multi,
            right: Box::from(group)
        };

        let result = ast.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }
}

