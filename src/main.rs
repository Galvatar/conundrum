use std::fs;

use crate::{interpreter::Interpreter, lexer::Lexer, parser::Parser};

mod token;
mod ast;
mod lexer;
mod parser;
mod interpreter;
mod value;

fn main() {
    // reading cdm extensions (ConnunDruM)
    let source = fs::read_to_string("./test.cdm").expect("Failed to read the source file!");
    
    // tokenise the souce text
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    // Parse the tokens into statements
    let mut parser = Parser::new(tokens);
    let stmt = parser.parse();

    // // interpret and execute the statements
    let interpreter = Interpreter::new(stmt);
    interpreter.interpret();

    // debug statement
    // let debug_string = format!("{:?}", stmt);
    // println!("{}", debug_string);
}