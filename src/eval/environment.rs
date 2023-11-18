use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::object::Object;

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Env>,
}

impl Environment {
    pub fn new_enclosed_environment(outer: &Env) -> Self {
        let mut env: Environment = Default::default();
        env.outer = Some(Rc::clone(outer));
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
