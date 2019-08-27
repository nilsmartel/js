use crate::vm::instruction::InstructionAddress;
use std::collections::HashMap;
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
