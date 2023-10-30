use parser::Parser;

use crate::lexer::Lexer;
use std::env;
use std::fs;

mod ast;
mod eval;
mod lexer;
mod parser;
mod token;

fn main() {
    if env::args().len() == 2 {
        let maybe_file = env::args().nth(1);
        let file = if let Some(f) = maybe_file {
            f
        } else {
            panic!("File not found, why my guy?")
        };

        let maybe_source_code = fs::read_to_string(file);

        let source_code = if maybe_source_code.is_ok() {
            maybe_source_code
        } else {
            panic!("File is empty? why would you give me an empty file")
        };

        let lexer = Lexer::new(source_code.unwrap());
        let mut pars = Parser::new(lexer);

        pars.read();
        pars.read();
        let program = pars.parse();
        dbg!(program);
    }

    if env::args().len() == 1 {
        let prompt = ">>";
        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        let mut lex = Lexer::new(line);

        while let Some(token) = lex.next() {
            println!("{:?}", token)
        }
    }
}
