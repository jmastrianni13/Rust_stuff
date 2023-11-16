use crate::expr;
use crate::scanner;
use crate::stmt;

pub struct Parser {
    tokens: Vec<scanner::Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<scanner::Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<stmt::Stmt>, String> {
        let mut stmts = vec![];
        let mut errs = vec![];

        while !self.is_at_end() {
            let stmt = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => errs.push(msg),
            }
        }

        if errs.len() == 0 {
            return Ok(stmts)
        } else {
            return Err(errs.join("\n"));
        }
    }

    fn declaration(&mut self) -> Result<stmt::Stmt, String> {
        if self.match_token(scanner::TokenType::Var) {
            match self.var_declaration() {
                Ok(statement) => {
                    return Ok(statement);
                },
                Err(msg) =>  {
                    self.synchronize();
                    return Err(msg);
                }
            }
        } else {
            return self.statement();
        }
    }

    fn var_declaration(&mut self) -> Result<stmt::Stmt, String> {
        let token = self.consume(scanner::TokenType::Identifier, "expected variable name")?;

        let initializer;
        if self.match_token(scanner::TokenType::Equal) {
            initializer = self.expression()?;
        } else {
            initializer = expr::Expr::Literal{ value: expr::LiteralValue::Nil };
        }

        self.consume(scanner::TokenType::Semicolon, "expected ';' after variable declaration")?;

        return Ok(stmt::Stmt::Var { name: token, initializer: initializer });
    }

    fn statement(&mut self) -> Result<stmt::Stmt, String> {
        if self.match_token(scanner::TokenType::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn print_statement(&mut self) -> Result<stmt::Stmt, String> {
        let value = self.expression()?;
        self.consume(scanner::TokenType::Semicolon, "Expected a ';' after value.")?;
        return Ok(stmt::Stmt::Print {
            expression: value
        });
    }

    fn expression_statement(&mut self) -> Result<stmt::Stmt, String> {
        let exp = self.expression()?;
        self.consume(scanner::TokenType::Semicolon, "Expected a ';' after expression.")?;
        return Ok(stmt::Stmt::Expression {
            expression: exp
        });

    }

    fn expression(&mut self) -> Result<expr::Expr, String> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<expr::Expr, String> {
        let mut exp = self.comparison()?;

        while self.match_tokens(&[scanner::TokenType::BangEqual, scanner::TokenType::EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            exp = expr::Expr::Binary {
                left: Box::from(exp),
                operator: operator,
                right: Box::from(rhs),
            };
        }
        return Ok(exp);
    }

    fn comparison(&mut self) -> Result<expr::Expr, String> {
        let mut exp = self.term()?;

        while self.match_tokens(
            &[
            scanner::TokenType::Greater,
            scanner::TokenType::GreaterEqual,
            scanner::TokenType::Less,
            scanner::TokenType::LessEqual,
            ]) {
                let op = self.previous();
                let rhs = self.term()?;
                exp = expr::Expr::Binary {
                    left: Box::from(exp),
                    operator: op,
                    right: Box::from(rhs),
                }
        }

        return Ok(exp);
    }

    fn term(&mut self) -> Result<expr::Expr, String> {
       let mut exp = self.factor()?;

       while self.match_tokens(&[
                               scanner::TokenType::Minus,
                               scanner::TokenType::Plus
       ]) {
           let op = self.previous();
           let rhs = self.factor()?;
           exp = expr::Expr::Binary {
               left: Box::from(exp),
               operator: op,
               right: Box::from(rhs),
           };
       }

       return Ok(exp);
    }

    fn factor(&mut self) -> Result<expr::Expr, String> {
        let mut exp = self.unary()?;
        while self.match_tokens(&[
                                scanner::TokenType::Slash,
                                scanner::TokenType::Star
        ]) {
            let op = self.previous();
            let rhs = self.unary()?;
            exp = expr::Expr::Binary {
                left: Box::from(exp),
                operator: op,
                right: Box::from(rhs),
            };
        }

        return Ok(exp);
    }

    fn unary(&mut self) -> Result<expr::Expr, String> {
        if self.match_tokens(&[
                             scanner::TokenType::Bang,
                             scanner::TokenType::Minus,
        ]) {
            let op = self.previous();
            let rhs = self.unary()?;
            return Ok(expr::Expr::Unary {
                operator: op,
                right: Box::from(rhs)
            });
        } else {
            return self.primary();
        }
    }

    fn primary(&mut self) -> Result<expr::Expr, String> {
        let token = self.peek();

        let result;
        match token.token_type {
            scanner::TokenType::LeftParen => {
                self.advance();
                let exp = self.expression()?;
                self.consume(scanner::TokenType::RightParen, "expected ')'")?;
                result = expr::Expr::Grouping {
                    expression: Box::from(exp)
                };
            },
            scanner::TokenType::False |
            scanner::TokenType::True |
            scanner::TokenType::Nil |
            scanner::TokenType::NumberLit |
            scanner::TokenType::StringLit => {
                self.advance();
                result = expr::Expr::Literal {
                    value: expr::LiteralValue::from_token(token)
                };
            },
            _ => return Err("expected expression".to_string()),
        }

        return Ok(result);
    }

    fn consume(&mut self, token_type: scanner::TokenType, msg: &str) -> Result<scanner::Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            return Ok(token);
        } else {
            return Err(msg.to_string());
        }
    }

    fn match_token(&mut self, typ: scanner::TokenType) -> bool {
        if self.is_at_end() {
            return false;
        } else {
            if self.peek().token_type == typ {
                self.advance();
                return true;
            } else {
                return false;
            }
        }
    }

    fn match_tokens(&mut self, typs: &[scanner::TokenType]) -> bool {
        for typ in typs {
            if self.match_token(*typ) {
                return true;
            }
        }

        return false;
    }

    fn advance(&mut self) -> scanner::Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();

    }

    fn peek(&mut self) -> scanner::Token {
        return self.tokens[self.current].clone();
    }

    fn previous(&mut self) -> scanner::Token {
        return self.tokens[self.current - 1].clone();
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().token_type == scanner::TokenType::Eof;
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == scanner::TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                scanner::TokenType::Class |
                scanner::TokenType::Fun |
                scanner::TokenType::Var |
                scanner::TokenType::For |
                scanner::TokenType::If |
                scanner::TokenType::While |
                scanner::TokenType::Print |
                scanner::TokenType::Return  => return,
                _ => (),
            }

            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::{LiteralValue, Scanner};

    #[test]
    fn test_addition() {
        let one = scanner::Token{
            token_type: scanner::TokenType::NumberLit,
            lexeme: "1".to_string(),
            literal: Some(LiteralValue::IntValue(1)),
            line_number: 0
        };

        let plus = scanner::Token{
            token_type: scanner::TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line_number: 0
        };

        let two = scanner::Token{
            token_type: scanner::TokenType::NumberLit,
            lexeme: "2".to_string(),
            literal: Some(LiteralValue::IntValue(2)),
            line_number: 0
        };

        let semicolon = scanner::Token{
            token_type: scanner::TokenType::Semicolon,
            lexeme: ";".to_string(),
            literal: None,
            line_number: 0
        };

        let tokens = vec![one, plus, two, semicolon];

        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        let string_exp = parsed_exp.to_string();

        assert_eq!(string_exp, "(+ 1 2)");

    }

    #[test]
    fn test_comparison () {
        let source = "1 + 2 == 5 + 7";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        let string_exp = parsed_exp.to_string();

        assert_eq!(string_exp, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_paren() {
        let source = "1 == (2 + 2)";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        let string_exp = parsed_exp.to_string();

        assert_eq!(string_exp, "(== 1 (group (+ 2 2)))");
    }

    #[test]
    fn test_order_of_op() {
        let source = "2 * 3 + 4";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        let string_exp = parsed_exp.to_string();

        assert_eq!(string_exp, "(+ (* 2 3) 4)");

        let source = "2 + 3 * 4";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        let string_exp = parsed_exp.to_string();

        assert_eq!(string_exp, "(+ 2 (* 3 4))");
    }
}

