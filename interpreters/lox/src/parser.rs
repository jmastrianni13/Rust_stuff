use crate::expr;
use crate::scanner;
use crate::stmt;

#[derive(Debug)]
enum FunctionKind {
    Function,
}

#[derive(Debug)]
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
                Err(msg) => {
                    errs.push(msg);
                    self.synchronize();
                }
            }
        }

        if errs.len() == 0 {
            return Ok(stmts);
        } else {
            return Err(errs.join("\n"));
        }
    }

    fn declaration(&mut self) -> Result<stmt::Stmt, String> {
        if self.match_token(scanner::TokenType::Var) {
            return self.var_declaration();
        } else if self.match_token(scanner::TokenType::Fun) {
            self.function(FunctionKind::Function)
        } else {
            return self.statement();
        }
    }

    fn function(&mut self, kind: FunctionKind) -> Result<stmt::Stmt, String> {
        let name = self.consume(
            scanner::TokenType::Identifier,
            &format!("expected {kind:?} name"),
        )?;

        self.consume(
            scanner::TokenType::LeftParen,
            &format!("expected '(' after {kind:?} name"),
        )?;

        let mut params = vec![];
        if !self.check(scanner::TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "line {location}: cannot have more 255 or more arguments"
                    ));
                }

                let param =
                    self.consume(scanner::TokenType::Identifier, "expected parameter name")?;
                params.push(param);

                if !self.match_token(scanner::TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(
            scanner::TokenType::RightParen,
            "expected ')' after parameters",
        )?;

        self.consume(
            scanner::TokenType::LeftBrace,
            &format!("expected '{{' before {kind:?} body"),
        )?;

        let body = match self.block_statement()? {
            stmt::Stmt::Block { statements } => statements,
            _ => panic!("block statement parsed something that was not a block"),
        };

        return Ok(stmt::Stmt::Function { name, params, body });
    }

    fn var_declaration(&mut self) -> Result<stmt::Stmt, String> {
        let token = self.consume(scanner::TokenType::Identifier, "expected variable name")?;

        let initializer;
        if self.match_token(scanner::TokenType::Equal) {
            initializer = self.expression()?;
        } else {
            initializer = expr::Expr::Literal {
                value: expr::LiteralValue::Nil,
            };
        }

        self.consume(
            scanner::TokenType::Semicolon,
            "expected ';' after variable declaration",
        )?;

        return Ok(stmt::Stmt::Var {
            name: token,
            initializer: initializer,
        });
    }

    fn statement(&mut self) -> Result<stmt::Stmt, String> {
        if self.match_token(scanner::TokenType::Print) {
            return self.print_statement();
        } else if self.match_token(scanner::TokenType::LeftBrace) {
            return self.block_statement();
        } else if self.match_token(scanner::TokenType::If) {
            return self.if_statement();
        } else if self.match_token(scanner::TokenType::While) {
            return self.while_statement();
        } else if self.match_token(scanner::TokenType::For) {
            return self.for_statement();
        } else if self.match_token(scanner::TokenType::Return) {
            return self.return_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn return_statement(&mut self) -> Result<stmt::Stmt, String> {
        let keyword = self.previous();
        let value;
        if !self.check(scanner::TokenType::Semicolon) {
            value = Some(self.expression()?);
        } else {
            value = None;
        }
        self.consume(
            scanner::TokenType::Semicolon,
            "expected ';' after return value.",
        )?;

        return Ok(stmt::Stmt::ReturnStmt { keyword, value });
    }

    fn for_statement(&mut self) -> Result<stmt::Stmt, String> {
        self.consume(scanner::TokenType::LeftParen, "expected '(' after 'for'")?;

        let initializer;

        if self.match_token(scanner::TokenType::Semicolon) {
            initializer = None;
        } else if self.match_token(scanner::TokenType::Var) {
            let var_decl = self.var_declaration()?;
            initializer = Some(var_decl);
        } else {
            let exp = self.expression_statement()?;
            initializer = Some(exp);
        }

        let condition;

        if !self.check(scanner::TokenType::Semicolon) {
            let exp = self.expression()?;
            condition = Some(exp);
        } else {
            condition = None;
        }
        self.consume(
            scanner::TokenType::Semicolon,
            "expected ';' after loop condition",
        )?;

        let increment;

        if !self.check(scanner::TokenType::RightParen) {
            let exp = self.expression()?;
            increment = Some(exp);
        } else {
            increment = None;
        }
        self.consume(
            scanner::TokenType::RightParen,
            "expected ')' after for clauses",
        )?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = stmt::Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(stmt::Stmt::Expression { expression: incr }),
                ],
            };
        }

        let cond;

        match condition {
            None => {
                cond = expr::Expr::Literal {
                    value: expr::LiteralValue::True,
                }
            }
            Some(c) => cond = c,
        }

        body = stmt::Stmt::WhileStmt {
            condition: cond,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = stmt::Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            };
        }

        return Ok(body);
    }

    fn while_statement(&mut self) -> Result<stmt::Stmt, String> {
        self.consume(scanner::TokenType::LeftParen, "expected '(' after 'while'")?;
        let condition = self.expression()?;
        self.consume(scanner::TokenType::RightParen, "expected ')' after 'while'")?;
        let body = self.statement()?;

        return Ok(stmt::Stmt::WhileStmt {
            condition,
            body: Box::new(body),
        });
    }

    fn if_statement(&mut self) -> Result<stmt::Stmt, String> {
        self.consume(scanner::TokenType::LeftParen, "expected '(' after 'if'")?;
        let predicate = self.expression()?;
        self.consume(
            scanner::TokenType::RightParen,
            "expected ')' after if-predicate",
        )?;

        let then = Box::new(self.statement()?);

        let els = if self.match_token(scanner::TokenType::Else) {
            let stm = self.statement()?;
            Some(Box::new(stm))
        } else {
            None
        };

        return Ok(stmt::Stmt::IfStmt {
            predicate,
            then,
            els,
        });
    }

    fn block_statement(&mut self) -> Result<stmt::Stmt, String> {
        let mut statements = vec![];

        while !self.check(scanner::TokenType::RightBrace) && !self.is_at_end() {
            let decl = self.declaration()?;
            statements.push(Box::new(decl));
        }

        self.consume(scanner::TokenType::RightBrace, "expected '}' after a block")?;

        return Ok(stmt::Stmt::Block { statements });
    }

    fn print_statement(&mut self) -> Result<stmt::Stmt, String> {
        let value = self.expression()?;
        self.consume(scanner::TokenType::Semicolon, "Expected a ';' after value.")?;
        return Ok(stmt::Stmt::Print { expression: value });
    }

    fn expression_statement(&mut self) -> Result<stmt::Stmt, String> {
        let exp = self.expression()?;
        self.consume(
            scanner::TokenType::Semicolon,
            "Expected a ';' after expression.",
        )?;
        return Ok(stmt::Stmt::Expression { expression: exp });
    }

    fn expression(&mut self) -> Result<expr::Expr, String> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<expr::Expr, String> {
        let exp = self.or()?;

        if self.match_token(scanner::TokenType::Equal) {
            let value = self.assignment()?;

            match exp {
                expr::Expr::Variable { name } => {
                    return Ok(expr::Expr::Assign {
                        name: name,
                        value: Box::from(value),
                    });
                }
                _ => Err("invalid assignment target.".to_string()),
            }
        } else {
            return Ok(exp);
        }
    }

    fn or(&mut self) -> Result<expr::Expr, String> {
        let mut exp = self.and()?;

        while self.match_token(scanner::TokenType::Or) {
            let operator = self.previous();
            let right = self.and()?;

            exp = expr::Expr::Logical {
                left: Box::new(exp),
                operator: operator,
                right: Box::new(right),
            };
        }

        return Ok(exp);
    }

    fn and(&mut self) -> Result<expr::Expr, String> {
        let mut exp = self.equality()?;

        while self.match_token(scanner::TokenType::And) {
            let operator = self.previous();
            let right = self.equality()?;
            exp = expr::Expr::Logical {
                left: Box::new(exp),
                operator: operator,
                right: Box::new(right),
            };
        }

        return Ok(exp);
    }

    fn equality(&mut self) -> Result<expr::Expr, String> {
        let mut exp = self.comparison()?;

        while self.match_tokens(&[
            scanner::TokenType::BangEqual,
            scanner::TokenType::EqualEqual,
        ]) {
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

        while self.match_tokens(&[
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

        while self.match_tokens(&[scanner::TokenType::Minus, scanner::TokenType::Plus]) {
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
        while self.match_tokens(&[scanner::TokenType::Slash, scanner::TokenType::Star]) {
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
        if self.match_tokens(&[scanner::TokenType::Bang, scanner::TokenType::Minus]) {
            let op = self.previous();
            let rhs = self.unary()?;
            return Ok(expr::Expr::Unary {
                operator: op,
                right: Box::from(rhs),
            });
        } else {
            return self.call();
        }
    }

    fn call(&mut self) -> Result<expr::Expr, String> {
        let mut exp = self.primary()?;

        loop {
            if self.match_token(scanner::TokenType::LeftParen) {
                exp = self.finish_call(exp)?;
            } else {
                break;
            }
        }

        return Ok(exp);
    }

    fn finish_call(&mut self, callee: expr::Expr) -> Result<expr::Expr, String> {
        let mut arguments = vec![];

        if !self.check(scanner::TokenType::RightParen) {
            loop {
                let arg = self.expression()?;
                arguments.push(arg);
                if arguments.len() >= 255 {
                    let location = self.peek().line_number;
                    return Err(format!(
                        "line {location}: cannot have more 255 or more arguments"
                    ));
                }

                if !self.match_token(scanner::TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(
            scanner::TokenType::RightParen,
            "expected a ')' after arguments.",
        )?;

        return Ok(expr::Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        });
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
                    expression: Box::from(exp),
                };
            }
            scanner::TokenType::False
            | scanner::TokenType::True
            | scanner::TokenType::Nil
            | scanner::TokenType::NumberLit
            | scanner::TokenType::StringLit => {
                self.advance();
                result = expr::Expr::Literal {
                    value: expr::LiteralValue::from_token(token),
                };
            }
            scanner::TokenType::Identifier => {
                self.advance();
                result = expr::Expr::Variable {
                    name: self.previous(),
                };
            }
            _ => return Err("expected expression".to_string()),
        }

        return Ok(result);
    }

    fn consume(
        &mut self,
        token_type: scanner::TokenType,
        msg: &str,
    ) -> Result<scanner::Token, String> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token = self.previous();
            return Ok(token);
        } else {
            return Err(msg.to_string());
        }
    }

    fn check(&mut self, typ: scanner::TokenType) -> bool {
        return self.peek().token_type == typ;
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
                scanner::TokenType::Class
                | scanner::TokenType::Fun
                | scanner::TokenType::Var
                | scanner::TokenType::For
                | scanner::TokenType::If
                | scanner::TokenType::While
                | scanner::TokenType::Print
                | scanner::TokenType::Return => return,
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
        let one = scanner::Token {
            token_type: scanner::TokenType::NumberLit,
            lexeme: "1".to_string(),
            literal: Some(LiteralValue::FValue(1.0)),
            line_number: 0,
        };

        let plus = scanner::Token {
            token_type: scanner::TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            line_number: 0,
        };

        let two = scanner::Token {
            token_type: scanner::TokenType::NumberLit,
            lexeme: "2".to_string(),
            literal: Some(LiteralValue::FValue(2.0)),
            line_number: 0,
        };

        let semicolon = scanner::Token {
            token_type: scanner::TokenType::Semicolon,
            lexeme: ";".to_string(),
            literal: None,
            line_number: 0,
        };

        let eof = scanner::Token {
            token_type: scanner::TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: 0,
        };

        let tokens = vec![one, plus, two, semicolon, eof];
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        assert_eq!(parsed_exp.len(), 1);
        let string_exp = parsed_exp[0].tostring();

        assert_eq!(string_exp, "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        assert_eq!(parsed_exp.len(), 1);
        let string_exp = parsed_exp[0].tostring();

        assert_eq!(string_exp, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_eq_with_paren() {
        let source = "1 == (2 + 2);";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        assert_eq!(parsed_exp.len(), 1);
        let string_exp = parsed_exp[0].tostring();

        assert_eq!(string_exp, "(== 1 (group (+ 2 2)))");
    }

    #[test]
    fn test_order_of_op() {
        let source = "2 * 3 + 4;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        assert_eq!(parsed_exp.len(), 1);
        let string_exp = parsed_exp[0].tostring();

        assert_eq!(string_exp, "(+ (* 2 3) 4)");

        let source = "2 + 3 * 4;";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();
        let tokens = scanner.tokens;
        let mut parser = Parser::new(tokens);
        let parsed_exp = parser.parse().unwrap();
        assert_eq!(parsed_exp.len(), 1);
        let string_exp = parsed_exp[0].tostring();

        assert_eq!(string_exp, "(+ 2 (* 3 4))");
    }
}
