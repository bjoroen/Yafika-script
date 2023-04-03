use crate::token::{Token, TokenType};

#[derive(PartialEq, Debug)]
pub struct Node {
    token: Token,
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return { value: Expression },
    StatmentExpression { value: Expression },
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Expression {
    Number(f64),
    PrefixExpression {
        Token: Token,
        Right: Box<Expression>,
    },
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
}
//
// #[derive(PartialEq, PartialOrd, Debug, Clone)]
// pub struct PrefixExpression {
//     pub Token: TokenType,
//     pub Operator: String,
//     pub Right: Expression,
// }

pub type Program = Vec<Statement>;
