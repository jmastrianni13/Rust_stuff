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

pub enum Expr {
    Binary { left: Box<Expr>, operator: scanner::Token, right: Box<Expr>},
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: scanner::Token, right: Box<Expr> },
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

        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }
}

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

