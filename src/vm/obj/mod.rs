mod arena;
pub use arena::Arena;

use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    String(Rc<String>),
    Number(f64),
    Reference(usize),
    FunctionPointer(usize),
}

impl Value {
    pub fn add(self, other: Value, _arena: &mut Arena<Object>) -> Value {
        use Value::*;

        match (self, other) {
            (Number(a), Number(b)) => Number(a + b),
            _ => unimplemented!(),
        }
    }
}

pub enum Object {
    Array(Vec<Value>),
    Class(std::collections::HashMap<String, Value>),
}
