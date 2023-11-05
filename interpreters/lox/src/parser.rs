use crate::scanner;

public struct Parser {
    tokens: Vec<scanner::Token>,
    current: usize,
}

