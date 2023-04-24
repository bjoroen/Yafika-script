#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub enum TokenType {
    // KeyWords
    If,
    Let,
    Fn,
    Else,
    Return,
    True,
    False,

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
    SemiColon,
    Comma,
    LeftParen,
    RightParen,
    RightBrace,
    LeftBrace,

    // END OF FILE
    EOF,
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
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
