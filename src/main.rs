use crate::lexer::Lexer;
use std::env;
use std::fs;

mod lexer;
mod token;

fn main() {
    let maybe_file = env::args().nth(1);
    let file = if let Some(f) = maybe_file{
        f
    } else {
        panic!("File not found, why my guy?")
    };

    let maybe_source_code = fs::read_to_string(file);

    let source_code = if maybe_source_code.is_ok() {
       maybe_source_code.unwrap() 
    } else {
       panic!("File is empty? why would you give me an empty file") 
    };

    let mut lexer = Lexer::new(source_code);

    lexer.lex()
}
