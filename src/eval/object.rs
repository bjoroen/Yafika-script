use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub enum Object {
    Integer(f64),
    Boolean(bool),
    Nil,
    Return(Box<Object>),
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Nil => write!(f, "null"),
            Object::Return(v) => write!(f, "{}", v),
        }
    }
}
