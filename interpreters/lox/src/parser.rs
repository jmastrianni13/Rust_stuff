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

        while self.match_token(scanner::TokenType::BangEqual, scanner::TokenType::EqualEqual) {
            let operator = self.previous();
            let rhs = self.comparison();
            let exp = expr::Expr::Binary {
                left: exp,
                operator: operator,
                right: rhs
            };

            return exp;

        }

        todo!();
    }

}

