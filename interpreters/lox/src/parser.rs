use crate::expr;
use crate::scanner;

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

    fn expression(&mut self) -> expr::Expr {
        self.equality()
    }

    fn equality(&mut self) -> expr::Expr {
        let mut exp = self.comparison();

        while self.match_tokens(&[scanner::TokenType::BangEqual, scanner::TokenType::EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison();
            exp = expr::Expr::Binary {
                left: Box::from(exp),
                operator: operator,
                right: Box::from(rhs),
            };
        }
        return exp;
    }

    fn comparison(&mut self) -> expr::Expr {
        let mut exp = self.term();

        while self.match_tokens(
            &[
            scanner::TokenType::Greater,
            scanner::TokenType::GreaterEqual,
            scanner::TokenType::Less,
            scanner::TokenType::LessEqual,
            ]) {
                let op = self.previous();
                let rhs = self.term();
                exp = expr::Expr::Binary {
                    left: Box::from(exp),
                    operator: op,
                    right: Box::from(rhs),
                }
        }

        return exp;
    }

    fn term(&mut self) -> expr::Expr {
       let mut exp = self.factor();

       while self.match_tokens(&[
                               scanner::TokenType::Minus,
                               scanner::TokenType::Plus
       ]) {
           let op = self.previous();
           let rhs = self.factor();
           exp = expr::Expr::Binary {
               left: Box::from(exp),
               operator: op,
               right: Box::from(rhs),
           };
       }

       return exp;
    }

    fn factor(&mut self) -> expr::Expr {
        let mut exp = self.unary();
        while self.match_tokens(&[
                                scanner::TokenType::Slash,
                                scanner::TokenType::Star
        ]) {
            let op = self.previous();
            let rhs = self.unary();
            exp = expr::Expr::Binary {
                left: Box::from(exp),
                operator: op,
                right: Box::from(rhs),
            };
        }

        return exp;
    }

    fn unary(&mut self) -> expr::Expr {
        if self.match_tokens(&[
                             scanner::TokenType::Bang,
                             scanner::TokenType::Minus,
        ]) {
            let op = self.previous();
            let rhs = self.unary();
            return expr::Expr::Unary {
                operator: op,
                right: Box::from(rhs)
            }
        } else {
            return self.primary();
        }
    }

    fn primary(&mut self) -> expr::Expr {
        if self.match_token(scanner::TokenType::LeftParen) {
            let exp = self.expression();
            self.consume(scanner::TokenType::RightParen, "expected ')'");
            return expr::Expr::Grouping {
                expression: Box::from(exp)
            };
        } else {
            let token = self.peek();
            self.advance();
            return expr::Expr::Literal {
                value: expr::LiteralValue::from_token(token)
            };
        }
    }

    fn consume(&mut self, token_type: scanner::TokenType, msg: &str) {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        } else {
            panic!("{}", msg);
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
}

