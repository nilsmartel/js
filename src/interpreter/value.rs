use std::{cell::RefCell, collections::HashMap, rc::Rc};

use dumpster::{unsync::Gc, Trace};

/// JavaScript Value aka. Object.
#[derive(Trace)]
pub enum Value {
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    String(/*immutable string type */ Gc<String>),
    Array(JsArray),
    Map(JsMap),
    // Function comes later, when I have more intuition about evaluation.
}

#[derive(Trace)]
pub struct JsArray(Gc<RefCell<Vec<Value>>>);

#[derive(Trace)]
pub struct JsMap(Gc<RefCell<HashMap<Value, Value>>>);
