// We need to import our Token definition (assuming you made one in token.rs)
#![allow(dead_code)]
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetType {
    String,
    Int,
}

/// Represents anything that produces a value
#[derive(Debug, Clone)]
pub enum Expr {
    /// A raw number, like `5.0`
    Literal(Literal),
    
    /// A variable name, like `x`
    Variable(String),
    
    /// A math operation: Left Side, Operator, Right Side
    /// Example: 5 + 3
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}

/// Represents an action that doesn't produce a value
#[derive(Debug, Clone)]
pub enum Stmt {
    /// An expression used as a statement (e.g., `5 + 3;`)
    Expression(Expr),
    
    /// A print statement (e.g., `print 5;`)
    Print(Expr),
    
    /// A variable declaration (e.g., `let x = 5;`)
    Var {
        name: String,
        var_type: TargetType,
        initializer: Expr,
    },
    
    /// An If statement (Condition, Then branch, optional Else branch)
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
    }
}