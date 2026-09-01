#![allow(dead_code)]

// lexer.rs
use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,   // The raw code, split into an array of characters
    tokens: Vec<Token>,  // The list of tokens we are building
    start: usize,        // The start of the current word being scanned
    current: usize,      // The current character we are looking at
    line: usize,         // What line of code are we on?
}

impl Lexer {
    // 1. Initialize a new Lexer
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(), // Convert String to Vec<char>
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    // 2. The main loop that processes the whole file
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next word
            self.start = self.current; 
            self.scan_token();
        }
        
        // Always append an End Of File token at the very end
        self.tokens.push(Token::new(TokenType::EOF, "".to_string(), self.line));
        self.tokens
    }

    // Helper: Have we processed all characters?
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // 3. Match characters to Tokens
    fn scan_token(&mut self) {
        let c = self.advance();
        
        match c {
            // Single character tokens
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),
            '=' => self.add_token(TokenType::Equal),
            
            // Ignore whitespace completely
            ' ' | '\r' | '\t' => {} 
            
            // If it's a newline, just increment our line counter
            '\n' => self.line += 1,
            
            // Catch-all for things we don't recognize yet
            _ => println!("Error: Unexpected character '{}' at line {}", c, self.line),
        }
    }

    // Helper: Consume the next character and return it
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    // Helper: Package the characters into a Token struct and save it
    fn add_token(&mut self, kind: TokenType) {
        // Grab the slice of characters from start to current
        let text: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(kind, text, self.line));
    }
}