use crate::token::Token;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    counter: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            counter: 0,
        }
    }

    pub fn lex(&mut self) {
        let tokens: Vec<Token> = Vec::new();

        while self.source.len() > self.counter {

            let c = self.current_char();
            match c {
                _ => println!("{}", c)

            }


            self.counter += 1 
        }
    }

    fn current_char(&self) -> char {
        *self.source.get(self.counter).unwrap()
    
    }
}
