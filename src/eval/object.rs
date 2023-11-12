use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use crate::ast::{BlockStatment, Expression};

pub type EvalError = String;

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let env: Environment = Default::default();
        env
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        self.store.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}

pub type Env = Rc<RefCell<Environment>>;
