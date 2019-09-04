use crate::vm::instruction::InstructionAddress;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::rc::Rc;

pub type RcString = Rc<String>;

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

impl Upcast<bool> for Object {
    fn upcast(self) -> Result<bool, ()> {
        use Object::*;
        match self {
            Boolean(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl Upcast<f64> for Object {
    fn upcast(self) -> Result<f64, ()> {
        use Object::*;
        match self {
            Boolean(b) => Ok(if b { 1.0 } else { 0.0 }),
            Number(n) => Ok(n),
            _ => Err(()),
        }
    }
}

impl Upcast<RcString> for Object {
    fn upcast(self) -> Result<RcString, ()> {
        use Object::*;
        match self {
            Boolean(b) => Ok(Rc::new(format!("{}", b))),
            Number(n) => Ok(Rc::new(format!("{}", n))),
            String(s) => Ok(s),
            Array(_) => Ok(Rc::new("[array]".to_string())),
            Map(_) => Ok(Rc::new("{object}".to_string())),
            Closure { .. } => Ok(Rc::new("function".to_string())),
        }
    }
}

/*
impl Add for Object {
    type Output = Object;

    fn add(self, o: Object) -> Object {
        use Object::*;

        match (self, o) {
            (String(s), o) => String(Rc::new(format!("{}{}", s, o.to_string()))),
            (s, String(o)) => String(Rc::new(format!("{}{}", s.to_string(), o))),
        }
    }
} */

trait Upcast<T> {
    fn upcast(self) -> Result<T, ()>;
}
