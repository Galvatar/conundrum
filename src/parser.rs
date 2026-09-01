#![allow(dead_code)]

use crate::token::{Token, TokenType};
use crate::ast::Expr; // Grab the Expr enum we defined earlier

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) {

    }

    fn parse_token(&mut self) {
        
    }

    // Inside impl Parser
    fn primary(&mut self) -> Expr {
        let token = self.advance();
        match token.kind {
            TokenType::Number => {
                let value: f64 = token.lexeme.parse().unwrap();
                Expr::Literal(value)
            }
            _ => panic!("Expected an expression at line {}", token.line),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        let t = self.tokens[self.current].clone();
        self.current += 1;
        t
    }

    fn match_token(&mut self, kind: TokenType) -> bool {
        self.peek().kind == kind
    }
}