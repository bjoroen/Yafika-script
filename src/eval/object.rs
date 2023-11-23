use std::fmt::Display;

use crate::ast::{BlockStatment, Expression};

use super::environment::Env;

pub type EvalError = String;

#[derive(PartialEq, Debug, Clone)]
#[allow(non_snake_case, dead_code)]
pub enum Object {
    Integer(f64),
    Boolean(bool),
    Nil,
    Return(Box<Object>),
    Function {
        Parameters: Option<Vec<Expression>>,
        Body: BlockStatment,
        env: Env,
    },
    Error(String),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "null"),
            Object::Return(v) => write!(f, "{}", v),
            Object::Error(e) => write!(f, "{}", e),
            Object::Function {
                Parameters,
                Body,
                env: _,
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
        }
    }
}

impl Object {
    pub fn type_info(&self) -> String {
        match self {
            Object::Integer(_) => format!("INT"),
            Object::Boolean(_) => format!("BOOLEAN"),
            Object::Nil => format!("Nil"),
            Object::Return(e) => e.type_info(),
            Object::Error(_) => format!("ERROR"),
            Object::Function {
                Parameters: _,
                Body: _,
                env: _,
            } => format!("FUNCTION"),
        }
    }
}
