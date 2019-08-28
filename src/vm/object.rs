use crate::vm::instruction::InstructionAddress;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::rc::Rc;

// TODO use GC
#[derive(Debug, Clone)]
struct Gc<T>(T);

/// Garbage Collected JavaScript Object
#[derive(Debug, Clone)]
pub enum Object {
    Boolean(bool),
    Number(f64),
    String(Rc<String>),
    Array(Gc<Vec<Object>>),
    Map(HashMap<Rc<String>, Object>),
    Closure {
        enviroment: Rc<Vec<Object>>,
        function: InstructionAddress,
    },
}

impl Object {
    fn to_string(&self) -> Rc<String> {
        use Object::*;
        match self {
            Boolean(b) => Rc::new(b.to_string()),
            Number(n) => Rc::new(n.to_string()),
            String(s) => s.clone(),
            _ => Rc::new("Object".to_string()),
        }
    }
}

impl Add for Object {
    type Output = Object;

    fn add(self, o: Object) -> Object {
        use Object::*;

        match (self, o) {
            (String(s), o) => String(Rc::new(format!("{}{}", s, o.to_string()))),
            (s, String(o)) => String(Rc::new(format!("{}{}", s.to_string(), o))),
        }
    }
}
