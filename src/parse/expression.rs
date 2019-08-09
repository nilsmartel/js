pub struct Value(f64);

pub enum Expr {
    Mutate(MutationKind, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
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
    Assign, // =
    AddAssign, // +=
    SubtractAssign, // -=
    ModAssign, // %=
    MulAssign, // *=
    DivAssign // /=
}

use crate::parse::tag_ws;
use nom::IResult;
impl Expr {
    pub fn parse(i: &str) -> IResult<&str, Expr> {
        tag_ws("<expr>")(i).map(|(rest, _)| (rest, Expr::Value(Value(0.0))))
    }
}

mod parse {
    use super::Expr;
    fn
}
