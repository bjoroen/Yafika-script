use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: Vec<char>,
    current: usize,
    next: usize,
    char: char,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut s = Self {
            tokens: Vec::new(),
            source: source.chars().collect(),
            current: 0,
            next: 1,
            char: '\0',
        };

        s.char = s.source[s.current];
        s
    }

    pub fn read(&mut self) {
        if self.next >= self.source.len() {
            self.char = '\0'
       } else {
            self.char = self.source[self.next]
        }

        self.current = self.next;
        self.next = self.current + 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.char.is_whitespace() {
            self.read()
        }
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.next >= self.source.len() {
            return None;
        }

        self.skip_whitespace();

        let token: Token = match self.char {
            // Assigne Token
            '=' => {
                self.read();
                Token::new(TokenType::Assign, "=".to_owned())
            }
            '+' => {
                self.read();
                Token::new(TokenType::Addition, "+".to_owned())
            }
            // String Token
            '\"' | '\'' => {
                let qoute_type = self.char;
                self.read();
                let mut buffer = String::new();

                while self.char != qoute_type {
                    buffer.push(self.char);
                    self.read();
                }
                self.read();
                Token::new(TokenType::String, buffer)
            }
            _ if self.char.is_numeric() => {
                let mut buffer: String = String::new();
                buffer.push(self.char);
                self.read();

                loop {
                    if self.current >= self.source.len() {
                        break;
                    }

                    if self.char == '_' {
                        self.read()
                    }

                    if !self.char.is_numeric() && self.char != '.' {
                        break;
                    }

                    buffer.push(self.char);
                    self.read()
                }

                Token::new(TokenType::Number, buffer)
            }
            // Let and Variable names
            _ if self.char.is_alphabetic() => {
                let mut buffer = String::new();

                buffer.push(self.char);

                self.read();

                while self.char.is_alphabetic() {
                    buffer.push(self.char);

                    self.read();
                }

                let token_type: TokenType = match buffer.as_str() {
                    "let" => TokenType::Let,
                    "if" => TokenType::If,
                    _ => TokenType::Identifier,
                };

                Token::new(token_type, buffer)
            }
            _ => {
                println!("{}", self.char);
                unimplemented!()
            }
        };

        self.read();
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let lexer = Lexer::new(String::from(
            "let x = 123
        let y = \"hello world\"
        let number = 420 + 69",
        ));
        let mut array_of_tokens: Vec<Token> = Vec::new();

        for t in lexer {
            println!("{:?}", t);
            array_of_tokens.push(t);
        }

        assert_eq!(array_of_tokens.len(), 14);
        assert_eq!(
            array_of_tokens[0],
            Token::new(TokenType::Let, "let".to_string())
        );
        assert_eq!(
            array_of_tokens[1],
            Token::new(TokenType::Identifier, "x".to_string())
        );
        assert_eq!(
            array_of_tokens[2],
            Token::new(TokenType::Assign, "=".to_string())
        );
        assert_eq!(
            array_of_tokens[3],
            Token::new(TokenType::Number, "123".to_string())
        );
        assert_eq!(
            array_of_tokens[4],
            Token::new(TokenType::Let, "let".to_string())
        );
        assert_eq!(
            array_of_tokens[5],
            Token::new(TokenType::Identifier, "y".to_string())
        );
        assert_eq!(
            array_of_tokens[6],
            Token::new(TokenType::Assign, "=".to_string())
        );
        assert_eq!(
            array_of_tokens[7],
            Token::new(TokenType::String, "hello world".to_string())
        );
        assert_eq!(
            array_of_tokens[12],
            Token::new(TokenType::Addition, "+".to_string())
        );
    }
}
