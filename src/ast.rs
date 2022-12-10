use crate::token::Token;

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

#[derive(PartialEq, Debug)]
pub enum Expression {
    Number(f64),
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
    Indexj,
}

pub type Program = Vec<Statement>;
