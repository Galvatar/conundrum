use crate::lexer::Lexer;

mod token;
mod ast;
mod lexer;
mod parser;

fn main() {
    let source: String = "var age = 128 + 35".to_string();
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    let debug_string = format!("{:?}", tokens);
    println!("{}", debug_string);
}
