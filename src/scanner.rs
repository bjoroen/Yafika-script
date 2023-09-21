use crate::{Token::Token, Token_Type::Token_Type};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

// new commit
impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&self) -> char {
        self.source.chars().take(self.current + 1).last().unwrap()
    }

    fn add_token(&mut self, token_type: Token_Type) {
        self.token_add(token_type)
    }

    fn token_add(&mut self, token_type: Token_Type) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, self.line))
    }

    fn match_char(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().take(self.current).last().unwrap() != expected {
            return false;
        }

        self.current + 1;
        true
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let s = String::new();
        self.tokens.push(Token::new(Token_Type::EOF, s, self.line));
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(Token_Type::LEFT_PAREN),
            ')' => self.add_token(Token_Type::RIGHT_PAREN),
            '{' => self.add_token(Token_Type::LEFT_BRACE),
            '}' => self.add_token(Token_Type::RIGHT_BRACE),
            ',' => self.add_token(Token_Type::COMMA),
            '.' => self.add_token(Token_Type::DOT),
            '-' => self.add_token(Token_Type::MINUS),
            '+' => self.add_token(Token_Type::PLUS),
            ';' => self.add_token(Token_Type::SEMICOLON),
            '*' => self.add_token(Token_Type::STAR),
            '!' => match self.match_char('=') {
                true => self.token_add(Token_Type::BANG_EQUAL),
                false => self.token_add(Token_Type::BANG),
            },
            // Should throw Yafika error
            _ => panic!("{}: {}", "Error at Line", self.line),
        }
    }
}
