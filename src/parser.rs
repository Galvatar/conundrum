#![allow(dead_code)]

use core::panic;

use crate::token::{Token, TokenType};
use crate::ast::{Expr, Stmt}; // Grab the Expr enum we defined earlier

pub struct Parser {
    tokens: Vec<Token>,
    statements: Vec<Stmt>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, statements: Vec::new() }
    }

    pub fn parse(mut self) -> Vec<Stmt> {
        while !self.is_at_end() {
            let peek = self.peek();
            if peek.kind == TokenType::If
                || peek.kind == TokenType::Var
                || peek.kind == TokenType::Print {
                let statement = self.statement();
                self.statements.push(statement);
            } else {
                let expression = self.expression();
                self.statements.push(Stmt::Expression(expression));
            }
        }

        self.statements
    }

    fn statement(&mut self) -> Stmt {
        let peek = self.peek();
        self.advance();
        match peek.kind {
            TokenType::Print => {
                let expr = self.expression();
                Stmt::Print(expr)
            }
            TokenType::If => {
                let expr = self.expression();
                let then = self.statement();
                Stmt::If { condition:expr, then_branch: Box::new(then), else_branch: None }
            }
            _ => {
                let name = self.advance();
                let equal = self.advance();
                if equal.kind == TokenType::Equal {
                    let expr = self.expression();
                    return Stmt::Var { name: name.lexeme, initializer: expr };
                }
                panic!("Invalid variable declaration structure");
            }
        }
    }

    fn expression(&mut self) -> Expr {
        let mut expression = self.factor();
        let mut peek = self.peek();
        while peek.kind == TokenType::Plus
            || peek.kind == TokenType::Minus {
            let operator = self.advance();
            let right = self.factor();
            expression = Expr::Binary { 
                left: Box::new(expression), 
                operator, 
                right: Box::new(right), 
            };
            peek = self.peek()
        }
        expression
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.primary();
        let mut peek = self.peek();
        while peek.kind == TokenType::Star
            || peek.kind == TokenType::Slash {
            let operator = self.advance();
            let right = self.primary();
            expr = Expr::Binary { 
                left: Box::new(expr), 
                operator, 
                right: Box::new(right), 
            };
            peek = self.peek()
        }
        expr
    }

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
        self.current >= self.tokens.len() || self.peek().kind == TokenType::EOF
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