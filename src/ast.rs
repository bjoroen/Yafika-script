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

impl Display for BlockStatment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let exprs = self
            .Statement
            .iter()
            .map(|exp| exp.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", exprs)
    }
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
        //TODO: I Dont think Option inside box is correct or a good way to handle this
        Right: Box<Option<Expression>>,
    },
    InfixExpression {
        Token: Token,
        Left: Box<Expression>,
        Op: Op,
        //TODO: I Dont think Option inside box is correct or a good way to handle this
        Right: Box<Option<Expression>>,
    },
    CallExpression {
        Token: Token,
        Function: Box<Expression>,
        //TODO: I dont think a vector of Options is correct or good, maybe a Option Vec instead?
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
            Op::Add => write!(f, "+"),
            Op::Subtract => write!(f, "-"),
            Op::Multiply => write!(f, "*"),
            Op::Divide => write!(f, "/"),
            Op::Bang => write!(f, "!"),
            Op::Equals => write!(f, "="),
            Op::NotEquals => write!(f, "!="),
            Op::Assign => write!(f, "="),
            Op::LessThan => write!(f, "<"),
            Op::GreaterThan => write!(f, ">"),
            Op::LessThanOrEquals => write!(f, "<="),
            Op::GreaterThanOrEquals => write!(f, ">="),
            Op::Call => write!(f, "()"),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let { name, value } => write!(f, "let {} = {}", name, value),
            Statement::Return { value } => write!(f, "return {}", value),
            Statement::StatmentExpression { value } => write!(f, "{}", value),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::String(s) => write!(f, "{}", s),
            Expression::Indentifier(i) => write!(f, "{}", i),
            Expression::Boolean(b) => write!(f, "{}", b),
            Expression::FunctionLiteral {
                Token: _,
                Parameters,
                Body,
            } => {
                let params = match Parameters {
                    Some(v) => v
                        .iter()
                        .map(|exp| exp.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    None => String::from(""),
                };

                write!(f, "fn({}) {{ {} }}", params, Body)
            }
            Expression::IfExpression {
                Token: _,
                Condition,
                Consequence,
                Alternative,
            } => {
                if let Some(else_block) = Alternative {
                    write!(
                        f,
                        "if({}) {{ {} }} else {{{}}}",
                        Condition, Consequence, else_block
                    )
                } else {
                    write!(f, "if({}){{ {} }}", Condition, Consequence)
                }
            }
            Expression::InfixExpression {
                Token: _,
                Left,
                Op,
                Right,
            } => {
                let right = match *Right.clone() {
                    Some(v) => v.to_string(),
                    None => todo!(),
                };
                write!(f, "{}{}{}", Left, Op, right)
            }
            _ => todo!(),
        }
    }
}

pub type Program = Vec<Statement>;
