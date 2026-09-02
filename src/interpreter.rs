use core::panic;
use std::{collections::HashMap};

use crate::{ast::{Expr, Stmt}, token::TokenType, value::RuntimeValue};

pub struct Interpreter {
    statements: Vec<Stmt>,
    variables: HashMap<String, RuntimeValue>,
    current: usize,
}

impl Interpreter {
    pub fn new(statements: Vec<Stmt>) -> Interpreter {
        Self { statements, current: 0, variables: HashMap::new() }
    }

    pub fn interpret(mut self) {
        while !self.is_at_end() {
            let stmt = self.advance();
            self.execute(&stmt);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr);
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                println!("{}", value);
            }
            Stmt::Var { name, initializer } => {
                let result = self.evaluate(initializer);
                self.variables.insert(name.clone(), result);
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_val = self.evaluate(condition);

                // Any non-zero float evaluates to true
                let is_truthy = match condition_val {
                    RuntimeValue::Literal(n) => n != 0.0,
                };

                if is_truthy {
                    for stmt in then_branch {
                        self.execute(stmt);
                    }
                } else if let Some(else_stmts) = else_branch {
                    for stmt in else_stmts {
                        self.execute(stmt);
                    }
                }
            }
        }
    }

    fn evaluate(&mut self, expr: &Expr) -> RuntimeValue {
        match expr {
            Expr::Literal(value) => {
                RuntimeValue::Literal(*value)
            }

            Expr::Binary { left, operator, right} => {
                let left_val = self.evaluate(left);
                let right_val = self.evaluate(right);
                
                // Extract numbers from runtime values to perform math
                let RuntimeValue::Literal(left_num) = left_val;
                let RuntimeValue::Literal(right_num) = right_val;

                let result = match operator.kind {
                    TokenType::Plus => left_num + right_num,
                    TokenType::Minus => left_num - right_num,
                    TokenType::Star => left_num * right_num,
                    TokenType::Slash => left_num / right_num,
                    _ => panic!("Unknown binary operator"),
                };

                RuntimeValue::Literal(result)
            }

            Expr::Variable(var) => {
                self.variables.get(var)
                    .expect("Variable has not been declared")
                    .clone()
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.statements.len()
    }

    // fn peek(&self) -> Stmt {
    //     self.statements[self.current].clone()
    // }

    fn advance(&mut self) -> Stmt {
        let t = self.statements[self.current].clone();
        self.current += 1;
        t
    }

    
}