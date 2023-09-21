#[derive(PartialEq, Debug)]
pub enum TokenType {
    // KeyWords
    If,
    Let,
    Fn,
    Else,
    Return,

    // Identifiers + litterals
    Identifier,
    Number,
    String,
    Nil,
    Bool,

    // Operators
    Assign,
    Addition,
    Minus,
    Star,
    Division,
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Delimiters
    Comma,
    LeftParen,
    RightParen,
    RightBrace,
    LeftBrace,
}

// This is good
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
