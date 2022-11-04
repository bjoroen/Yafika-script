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

        let token: Token = match self.char {
            // Assigne Token
            '=' => {
                self.read();
                Token::new(TokenType::Assign, "=".to_owned())
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
            _ => unimplemented!(),
        };

        self.read();
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_lexer() {
        let lexer = Lexer::new(String::from("let"));

        for t in lexer {
            assert_eq!(t, Token::new(TokenType::Let, "let".to_string()));
        }
    }

    #[test]
    fn identifier_lexer() {
        let lexer = Lexer::new(String::from("Yafika"));

        for t in lexer {
            assert_eq!(t, Token::new(TokenType::Identifier, "Yafika".to_string()));
        }
    }

    #[test]
    fn assigne_lexer() {
        let lexer = Lexer::new(String::from("="));

        for t in lexer {
            assert_eq!(t, Token::new(TokenType::Assign, "=".to_string()));
        }
    }

    #[test]
    fn string_lexer() {
        let lexer = Lexer::new(String::from("\"Nandi\""));
        let single_qoute_lexer = Lexer::new(String::from("\'IsCute\'"));

        for t in lexer {
            assert_eq!(t, Token::new(TokenType::String, "Nandi".to_string()));
        }

        for t in single_qoute_lexer {
            assert_eq!(t, Token::new(TokenType::String, "IsCute".to_string()));
        }
    }
    #[test]

    fn number_lexer() {
        let lexer = Lexer::new(String::from("12312"));

        for t in lexer {
            assert_eq!(t, Token::new(TokenType::Number, "12312".to_string()));
        }
    }
    #[test]
    fn number_with_dot_lexer() {
        let lexer = Lexer::new(String::from("1231.2"));

        for t in lexer {
            assert_eq!(t, Token::new(TokenType::Number, "1231.2".to_string()));
        }
    }
    #[test]
    fn number_with_underscore_lexer() {
        let lexer = Lexer::new(String::from("100_000"));

        for t in lexer {
            assert_eq!(t, Token::new(TokenType::Number, "100000".to_string()));
        }
    }
}
