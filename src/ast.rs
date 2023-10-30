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

#[allow(non_snake_case)]
#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct BlockStatment {
    pub Statement: Vec<Statement>,
}

#[allow(non_snake_case)]
#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Expression {
    Number(f64),
    String(String),
    Indentifier(String),
    Boolean(bool),
    FunctionLiteral {
        Token: Token,
        Parameters: Option<Vec<Expression>>,
        Body: BlockStatment,
    },
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
    CallExpression {
        Token: Token,
        Function: Box<Expression>,
        Arguments: Vec<Option<Expression>>,
    },
}

#[allow(dead_code)]
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
            TokenType::LeftParen => Precedence::Call,
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
    Call,
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
            TokenType::LeftParen => Self::Call,
            _ => unreachable!("{:?}", token_type),
        }
    }
}

pub type Program = Vec<Statement>;
