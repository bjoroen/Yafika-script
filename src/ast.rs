use crate::token::{Token, TokenType};

#[derive(PartialEq, Debug)]
pub struct Node {
    token: Token,
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return { value: Expression },
    StatmentExpression { value: Expression },
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct BlockStatment {
    pub Token: Token,
    pub Statement: Vec<Statement>,
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Expression {
    Number(f64),
    Indentifier(String),
    Boolean(bool),
    IfExpression {
        Token: Token,
        Condition: Box<Expression>,
        Consequence: BlockStatment,
        Alternative: Option<BlockStatment>,
    },
    PrefixExpression {
        Token: Token,
        Op: Op,
        Right: Box<Option<Expression>>,
    },
    InfixExpression {
        Token: Token,
        Left: Box<Expression>,
        Op: Op,
        Right: Box<Option<Expression>>,
    },
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Precedence {
    pub fn get_precedence(token_type: &TokenType) -> Self {
        match token_type {
            TokenType::EqualEqual | TokenType::BangEqual => Precedence::Equals,
            TokenType::Greater | TokenType::Less => Precedence::LessGreater,
            TokenType::Addition | TokenType::Minus => Precedence::Sum,
            TokenType::Division | TokenType::Star => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    // Modulo,
    Bang,
    Equals,
    NotEquals,
    Assign,
    LessThan,
    GreaterThan,
    LessThanOrEquals,
    GreaterThanOrEquals,
    // And,
    // Or,
    // Pow,
    // In,
    // NotIn,
}

impl Op {
    pub fn token(token_type: &TokenType) -> Self {
        match token_type {
            TokenType::Addition => Self::Add,
            TokenType::Minus => Self::Subtract,
            TokenType::Star => Self::Multiply,
            TokenType::Division => Self::Divide,
            TokenType::Bang => Self::Bang,
            TokenType::EqualEqual => Self::Equals,
            TokenType::BangEqual => Self::NotEquals,
            TokenType::Assign => Self::Assign,
            TokenType::Less => Self::LessThan,
            TokenType::Greater => Self::GreaterThan,
            TokenType::LessEqual => Self::LessThanOrEquals,
            TokenType::GreaterEqual => Self::GreaterThanOrEquals,
            _ => unreachable!("{:?}", token_type),
        }
    }
}

pub type Program = Vec<Statement>;
