use crate::vm::instruction::InstructionAddress;
mod arena;
mod upcast;
pub use arena::Arena;
pub use upcast::Upcast;

use std::rc::Rc;

#[derive(Debug)]
pub enum Object {
    Array(Vec<Value>),
    Map(std::collections::HashMap<String, Value>),
    Closure {
        enviroment: Vec<Value>,
        function_pointer: InstructionAddress,
    },
}

#[derive(Clone, Debug)]
pub enum Value {
    Undefined,
    Null,
    Boolean(bool),
    String(Rc<String>),
    Number(f64),
    Reference(usize),
    FunctionPointer(InstructionAddress),
}

impl Value {
    pub fn add(self, other: Value, _arena: &mut Arena<Object>) -> Value {
        use Value::*;

        match (self, other) {
            (Boolean(a), Boolean(b)) => Number(((a as u32) + (b as u32)) as f64),
            (Number(a), Number(b)) => Number(a + b),
            _ => Undefined,
        }
    }

    pub fn get(self, key: Value, arena: &mut Arena<Object>) -> Value {
        match self {
            Value::Reference(addr) => {
                let obj = &arena.objects()[addr];
                match (obj, key) {
                    (Object::Array(a), Value::Number(i)) => a[i as usize].clone(),
                    (Object::Map(m), Value::String(k)) => m
                        .get(&k.to_string())
                        .map_or(Value::Undefined, |v| v.clone()),
                    _ => Value::Undefined,
                }
            }
            _ => Value::Undefined,
        }
    }
}

impl Upcast<bool> for Value {
    fn upcast(&self) -> Result<bool, ()> {
        use Value::*;
        match self {
            Boolean(b) => Ok(*b),
            Number(n) => Ok(*n == 1.0),
            String(_) | Undefined | Null => Ok(false),
            _ => Ok(false),
        }
    }
}

impl Upcast<f64> for Value {
    fn upcast(&self) -> Result<f64, ()> {
        use Value::*;
        match self {
            Boolean(b) => Ok((*b as u32) as f64),
            Number(n) => Ok(*n),
            _ => Err(()),
        }
    }
}

impl Upcast<Rc<String>> for Value {
    fn upcast(&self) -> Result<Rc<String>, ()> {
        use Value::*;
        match self {
            // TODO easy to optimize without using the stack
            Undefined => Ok(Rc::new("undefined".to_string())),
            Null => Ok(Rc::new("null".to_string())),
            Boolean(b) => Ok(Rc::new(format!("{}", b))),
            Number(n) => Ok(Rc::new(format!("{}", n))),
            String(s) => Ok(s.clone()),
            // TODO arrays behave different from this (entries.join(','))
            Reference(_) => Ok(Rc::new("[object Object]".to_string())),
            FunctionPointer(_) => Ok(Rc::new("function".to_string())),
        }
    }
}
