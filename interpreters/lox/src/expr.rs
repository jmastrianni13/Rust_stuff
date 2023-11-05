use crate::scanner;


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

