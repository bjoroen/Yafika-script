use std::{env, fs::File, io::prelude::*, str};

mod Token;
mod Token_Type;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("{}", args.len());
        println!("Usage: Yafika [script]");
        panic!("Panic and stuff")
    }

    if args.len() == 2 {
        run_file(&args[1]);
    }
}

pub struct Yafika {
    pub had_error: bool,
}

impl Yafika {
    pub fn error(&mut self, line: i32, message: String) {
        self.report(line, "", message)
    }

    pub fn report(&mut self, line: i32, at: &str, message: String) {
        println!("{} line Error {}: {}", line, at, message);
        self.had_error = true;
    }
}

pub fn run_file(file: &String) {
    let mut f = File::open(file).expect("No File Found!");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).expect("something went wrong");

    let string_from_vec = String::from_utf8(buffer);
    run(string_from_vec.unwrap())
}

pub fn run(source: String) {
    let tokens: Vec<String> = source.split(" ").map(String::from).collect();
    println!("{:#?}", tokens)
}
