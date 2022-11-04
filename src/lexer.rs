use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: Vec<char>,
    counter: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            tokens: Vec::new(),
            source: source.chars().collect(),
            counter: 0,
        }
    }

    pub fn lex(&mut self) {

        while self.source.len() > self.counter {
            let c = self.current_char();
            match c {
                // Assigne Token
                '=' => {
                    self.tokens.push(Token::new(TokenType::Assign, "=".to_owned()));
                    self.counter += 1;
                }
                // String Token
                '\"' | '\'' => {
                    let qoute_type = self.current_char();
                    self.counter += 1;
                    let mut buffer = String::new();

                    while self.current_char() != qoute_type {
                        buffer.push(self.current_char());
                        self.counter += 1;
                    }
                    self.tokens.push(Token::new(TokenType::String, buffer));
                    self.counter += 1;
                }
                // Let and Variable names
                _ if c.is_alphabetic() => {
                    let mut buffer = String::new();

                    buffer.push(c);

                    self.counter += 1;

                    while self.current_char().is_alphabetic() {
                        buffer.push(self.current_char());
                        self.counter += 1;
                    }

                    let token_type: TokenType = match buffer.as_str() {
                        "let" => TokenType::Let,
                        _ => TokenType::Identifier,
                    };

                    self.tokens.push(Token::new(token_type, buffer))
                }
                _ => {
                    self.counter += 1;
                }
            };

            self.counter += 1
        }
        println!("{:?}", self.tokens);
    }

    fn current_char(&self) -> char {
        *self.source.get(self.counter).unwrap()
    }
}
