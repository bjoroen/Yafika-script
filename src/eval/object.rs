use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

pub type EvalError = String;

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Object {
    Integer(f64),
    Boolean(bool),
    Nil,
    Return(Box<Object>),
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
        }
    }
}

#[derive(Default, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env: Environment = Default::default();
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
