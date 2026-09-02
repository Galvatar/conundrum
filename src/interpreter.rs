use core::panic;
use std::{collections::HashMap};

use crate::{ast::{Expr, Literal, Stmt, TargetType}, token::TokenType, value::RuntimeValue};

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
            Stmt::Var { name, initializer, var_type } => {
                let result = self.evaluate(initializer);
                match (&var_type, &result) {
                    (TargetType::Int, RuntimeValue::Number(_)) => {},
                    (TargetType::String, RuntimeValue::String(_)) => {},
                    (expected, actual) => {
                        panic!(
                            "TypeError: Cannot assign value of type {:?} to variable '{}' declared as {:?}",
                            actual, name, expected
                        );
                    }
                }
                self.variables.insert(name.clone(), result);
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let condition_val = self.evaluate(condition);

                // Any non-zero float evaluates to true
                let is_truthy = match condition_val {
                    RuntimeValue::Number(n) => n != 0.0,
                    RuntimeValue::String(val) => !val.is_empty()
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
                match value {
                    Literal::Number(count) => RuntimeValue::Number(*count),
                    Literal::String(str) => RuntimeValue::String(str.clone())
                }
            }

            Expr::Binary { left, operator, right} => {
                let left_val = self.evaluate(left);
                let right_val = self.evaluate(right);

                match (left_val, &operator.kind, right_val) {
                    // Number arithmetic
                    (RuntimeValue::Number(l), TokenType::Plus, RuntimeValue::Number(r)) => RuntimeValue::Number(l + r),
                    (RuntimeValue::Number(l), TokenType::Minus, RuntimeValue::Number(r)) => RuntimeValue::Number(l - r),
                    (RuntimeValue::Number(l), TokenType::Star, RuntimeValue::Number(r)) => RuntimeValue::Number(l * r),
                    (RuntimeValue::Number(l), TokenType::Slash, RuntimeValue::Number(r)) => RuntimeValue::Number(l / r),

                    // String concatenation
                    (RuntimeValue::String(l), TokenType::Plus, RuntimeValue::String(r)) => {
                        RuntimeValue::String(format!("{}{}", l, r))
                    }

                    // Mismatched types or unsupported operations (e.g., subtracting strings)
                    (left_type, op, right_type) => panic!(
                        "TypeError: Cannot apply operator '{:?}' between {:?} and {:?}",
                        op, left_type, right_type
                    ),
                }
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