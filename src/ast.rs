use crate::token::{Token, TokenType};
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Node {
    Program(Program),
    Statment(Statement),
    Expression(Expression),
    BlockStatment(BlockStatment),
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

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "{}", self),
            Op::Subtract => write!(f, "{}", self),
            Op::Multiply => write!(f, "{}", self),
            Op::Divide => write!(f, "{}", self),
            Op::Bang => write!(f, "{}", self),
            Op::Equals => write!(f, "{}", self),
            Op::NotEquals => write!(f, "{}", self),
            Op::Assign => write!(f, "{}", self),
            Op::LessThan => write!(f, "{}", self),
            Op::GreaterThan => write!(f, "{}", self),
            Op::LessThanOrEquals => write!(f, "{}", self),
            Op::GreaterThanOrEquals => write!(f, "{}", self),
            Op::Call => write!(f, "{}", self),
        }
    }
}

pub type Program = Vec<Statement>;
