use std::{env, str, fs::File, io::prelude::*};

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

pub fn run_file(file: &String) {
    let mut f = File::open(file).expect("No File Found!");
    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer).expect("something went wrong");

    let string_from_vec = str::from_utf8(&buffer);
    run(string_from_vec.unwrap())
}


pub fn run(source: &str) {
    println!("{}", source)
}
