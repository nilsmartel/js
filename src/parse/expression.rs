pub struct Value(f64);
impl Value {
    pub fn parse(input: &str) -> IResult<&str, Value> {
        crate::parse::char_ws('1')(input).map(|(i, _)| (i, Value(1.0)))
    }

    fn as_expr(self) -> Expr {
        Expr::Value(self)
    }
}

pub enum Expr {
    Mutate(MutationKind, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Exponent(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Value(Value),
    // TODO bitshift
}

enum MutationKind {
    Assign,         // =
    AddAssign,      // +=
    SubtractAssign, // -=
    ModAssign,      // %=
    MulAssign,      // *=
    DivAssign,      // /=
}

use crate::parse::tag_ws;
use nom::IResult;
impl Expr {
    pub fn parse(i: &str) -> IResult<&str, Expr> {
        tag_ws("<expr>")(i).map(|(rest, _)| (rest, Expr::Value(Value(0.0))))
    }
}

mod parse {
    use super::*;
    use crate::parse::{char_ws, tag_ws};
    fn preceding_not(i: &str) -> IResult<&str, Expr> {
        if let Ok((i, _)) = char_ws('-')(i) {
            let (i, e) = exponent(i)?;
            return Ok((i, Expr::Not(Box::new(e))));
        }

        value(i)
    }

    fn exponent(input: &str) -> IResult<&str, Expr> {
        unimplemented!();
        // TODO use parse::util::concat here!
    }

    fn value(i: &str) -> IResult<&str, Expr> {
        // TODO Alterantive Case of (nestings) here!!
        Value::parse(i).map(|(i, v)| (i, v.as_expr()))
    }
}
