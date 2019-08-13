use crate::parse::{expression::Expr, scope::Function};
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};
use std::collections::HashMap;

/// Represents Parsed JavaScript Object.
/// Note, that this is _not_ it's final representation,
/// just an Building Block
#[derive(Debug)]
pub enum Object {
    Boolean(bool),
    Float(f64),
    String(String),
    Array(Vec<Expr>),
    Map(HashMap<String, Expr>),
    Closure(Function),
}

impl Object {
    pub fn parse(input: &str) -> IResult<&str, Object> {
        unimplemented!();
    }

    fn p_bool(input: &str) -> IResult<&str, Object> {
        alt((
            map(tag("true"), |_| Object::Boolean(true)),
            map(tag("false"), |_| Object::Boolean(false)),
        ))(input)
    }

    pub fn as_expr(self) -> Expr {
        Expr::Value(self)
    }
}
