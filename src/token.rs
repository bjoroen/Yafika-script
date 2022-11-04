
#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Assign,
    Let,
    String,
    If,
    Number,
}


#[derive(PartialEq)]
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}
