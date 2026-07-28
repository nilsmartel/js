use std::{cell::Cell, collections::HashMap, rc::Rc};




/// JavaScript Value aka. Object.
pub enum Value {
    Bool(bool),
    Number(f64),
    String(/*immutable string type */ Rc<String>),
    Array(JsArray),
    Map(JsMap),
    // Function comes later, when I have more intuition about evaluation.
}

pub struct JsArray(Rc<Cell<Vec<Value>>>);

pub struct JsMap(Rc<Cell<HashMap<Value, Value>>>);
