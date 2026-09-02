#![allow(dead_code)]

/// The different categories of words our language understands
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Equal,      // =
    
    // Literals (Values)
    Number,     // e.g., 5.0
    String,
    Identifier, // e.g., variable names like "x"

    // Brackets
    OpenSmooth, // (
    ClosingSmooth, // )
    OpenCurly, // {
    ClosingCurly, // }
    
    // Keywords
    StringVar,        // "string"
    IntVar,         // "int"
    If,         // "if"
    Print,      // "print"
    
    // End of File
    NewLine,
    EOF,
}

/// A specific word found in the source code
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType, // What category is it?
    pub lexeme: String,  // The actual text (e.g., "+", "let", "my_var")
    pub line: usize,     // What line number was it on? (Helpful for error messages)
}

impl Token {
    // A quick helper function to create a new token
    pub fn new(kind: TokenType, lexeme: String, line: usize) -> Self {
        Self { kind, lexeme, line }
    }
}