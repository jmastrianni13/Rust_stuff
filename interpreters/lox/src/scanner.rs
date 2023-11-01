pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        return Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(msg) => errors.push(msg),
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            line_number: self.line,
        });

        if errors.len() > 0 {
            let mut joined = "".to_string();
            for error in errors {
                joined.push_str(&error);
                joined.push_str("\n");
            };
            return Err(joined);
        }

        return Ok(self.tokens.clone());
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let token = if self.char_match('=') {
                    // !=
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token);
            },
            '=' => {
                let token = if self.char_match('=') {
                    // !=
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token);
            },
            '<' => {
                let token = if self.char_match('=') {
                    // !=
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token);
            },
            '>' => {
                let token = if self.char_match('=') {
                    // !=
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token);
            },
            '/' => {
                if self.char_match('/') {
                    loop {
                        if self.peek() == '\n' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            },
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1,
            '"' => self.string()?,
            _ => return Err(format!("unrecognized char at line {}: {}", self.line, c))
        }

        return Ok(());
    }

    fn string(self: &mut Self) -> Result<(), String> {
        // "some string wrapped in double quotes"
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("Unterminated string".to_string());
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.add_token_lit(TokenType::StringLit, Some(LiteralValue::StringValue(value.to_string())));

        return Ok(());

    }

    fn peek(self: &Self) -> char {
        if self.is_at_end() {
            return '\0'; // null character
        } else {
            return self.source.chars().nth(self.current).unwrap();
        }
    }

    fn char_match(self: &mut Self, ch: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != ch {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    fn advance(self: &mut Self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;

        return c;
    }

    fn add_token(self: &mut Self, token_type: TokenType) {
        self.add_token_lit(token_type, None);
    }

    fn add_token_lit(self: &mut Self, token_type: TokenType, literal: Option<LiteralValue>) {
        let mut text = "".to_string();
        let lit = self.source[self.start..self.current]
            .chars()
            .map(|ch| text.push(ch));

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            literal: literal,
            line_number: self.line,
        });
    }

    fn is_at_end(self: &Self) -> bool {
        return self.current >= self.source.len();
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line_number: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line_number: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line_number,
        }
    }

    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal)

    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    IdentifierVal(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier,
    StringLit,
    Number,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "(( )) }{";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 7);
        assert_eq!(scanner.tokens[0].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[1].token_type, TokenType::LeftParen);
        assert_eq!(scanner.tokens[2].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[3].token_type, TokenType::RightParen);
        assert_eq!(scanner.tokens[4].token_type, TokenType::RightBrace);
        assert_eq!(scanner.tokens[5].token_type, TokenType::LeftBrace);
        assert_eq!(scanner.tokens[6].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != == >=";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 5);
        assert_eq!(scanner.tokens[0].token_type, TokenType::Bang);
        assert_eq!(scanner.tokens[1].token_type, TokenType::BangEqual);
        assert_eq!(scanner.tokens[2].token_type, TokenType::EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, TokenType::GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, TokenType::Eof);
    }

    #[test]
    fn handle_string_literal() {
        //let source = r#""ABC""#; // just another way to have nested quotes
        let source = "\"ABC\"";
        let mut scanner = Scanner::new(source);
        scanner.scan_tokens().unwrap();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::StringLit);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal type"),
        }
    }

    #[test]
    fn handle_string_literal_not_term() {
        //let source = r#""ABC"#; // just another way to have nested quotes
        let source = "\"ABC";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        match result {
            Err(_) => (),
            _ => panic!("unterminated string did not raise error"),
        }

    }

    #[test]
    fn handle_string_literal_multi_line() {
        let source = "\"ABC\ndef\n123\"";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, TokenType::StringLit);
        assert_eq!(scanner.tokens[1].token_type, TokenType::Eof);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            LiteralValue::StringValue(val) => assert_eq!(val, "ABC\ndef\n123"),
            _ => panic!("Incorrect literal type"),
        }
    }
}


