use eval::environment::Env;
use parser::Parser;

use crate::eval::evaluator;
use crate::lexer::Lexer;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;

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
            panic!("File is empty?? why would you give me an empty file")
        };

        let lexer = Lexer::new(source_code.unwrap());
        let mut pars = Parser::new(lexer);

        pars.read();
        pars.read();
        let program = pars.parse();
        let ev: Env = Rc::new(RefCell::new(Default::default()));
        let evaluation = evaluator::eval(ast::Node::Program(program.clone()), &ev);
        match evaluation {
            Ok(v) => println!("{}", v.to_string()),
            Err(e) => println!("{}", e),
        }
        // println!("{}", evaluation.to_string())
    }
}
