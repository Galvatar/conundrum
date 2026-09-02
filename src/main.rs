use std::fs;

use crate::{lexer::Lexer, parser::Parser};

mod token;
mod ast;
mod lexer;
mod parser;

fn main() {
    let source = fs::read_to_string("./test.cdm").expect("Failed to read the source file!");
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let parser = Parser::new(tokens);
    let stmt = parser.parse();
    let debug_string = format!("{:?}", stmt);
    println!("{}", debug_string);
}