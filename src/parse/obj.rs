use crate::parse::{expression::Expr, scope::Function};
use std::collections::HashMap;

/// Represents Parsed JavaScript Object.
/// Note, that this is _not_ it's final representation,
/// just an Building Block
pub enum Object {
    Boolean(bool),
    Float(f64),
    String(String),
    Array(Vec<Expr>),
    Map(HashMap<String, Expr>),
    Closure(Function),
}
