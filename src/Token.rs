use std::fmt::format;

use crate::Token_Type::Token_Type;

pub struct Token {
    pub token_type: Token_Type,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: Token_Type, lexeme: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }

    // pub fn toString(&mut self) -> String {}
}
