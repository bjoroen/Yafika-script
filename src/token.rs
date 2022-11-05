#[derive(PartialEq, Debug)]
pub enum TokenType {
    Identifier,
    Assign,
    Let,
    String,
    If,
    Number,
    Addition,
    Minus,
    
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}
