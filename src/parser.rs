#![allow(dead_code)]

use core::panic;

use crate::token::{Token, TokenType};
use crate::ast::{Expr, Literal, Stmt, TargetType}; // Grab the Expr enum we defined earlier

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if self.match_token(TokenType::NewLine) {
                self.advance();
                continue;
            } else if self.match_token(TokenType::If)
                || self.match_token(TokenType::StringVar)
                || self.match_token(TokenType::IntVar)
                || self.match_token(TokenType::Print) {
                statements.push(self.statement());
            } else if self.match_token(TokenType::OpenCurly) {
                self.advance();
                let block_stmts = self.parse();
                statements.extend(block_stmts);
            } else if self.match_token(TokenType::ClosingCurly) {
                self.advance();
                break;
            } else {
                let expression = self.expression();
                statements.push(Stmt::Expression(expression));
            }
        }

        statements
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token(TokenType::Print) {
            self.advance();
            let expr = self.expression();
            Stmt::Print(expr)
        } else if self.match_token(TokenType::If) {
            self.advance();
            let expr = self.expression();
            let then = self.parse();
            Stmt::If { condition:expr, then_branch: then, else_branch: None }
        } else if self.match_token(TokenType::StringVar) || self.match_token(TokenType::IntVar) {
            let token = self.peek();
            self.advance();
            let name = self.advance();
            let equal = self.advance();
            if equal.kind == TokenType::Equal {
                let expr = self.expression();
                let target = match token.kind {
                    TokenType::StringVar => TargetType::String,
                    TokenType::IntVar => TargetType::Int,
                    _ => panic!("Could not find declared variable type")
                };
                return Stmt::Var { name: name.lexeme, initializer: expr, var_type: target };
            }
            panic!("Invalid variable declaration structure");
        } else {
            let debug_string = format!("{:?}", self.peek());
            println!("{}", debug_string);
            panic!("Unexpected statement token at line {}", self.peek().line);
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
                Expr::Literal(Literal::Number(value))
            }
            TokenType::String => {
                Expr::Literal(Literal::String(token.lexeme))
            }
            TokenType::Identifier => {
                Expr::Variable(token.lexeme)
            }
            TokenType::OpenSmooth => {
                let expr = self.expression();
                let closing = self.advance();
                if closing.kind != TokenType::ClosingSmooth {
                    panic!("Expected ')' after expression at line {}", closing.line);
                }
                expr
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