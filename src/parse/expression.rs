#[derive(Debug)]
pub struct Value(f64);
impl Value {
    pub fn parse(input: &str) -> IResult<&str, Value> {
        crate::parse::char_ws('1')(input).map(|(i, _)| (i, Value(1.0)))
    }

    fn as_expr(self) -> Expr {
        Expr::Value(self)
    }
}

#[derive(Debug)]
pub enum Expr {
    Mutate(MutationKind, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Exponent(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Value(Value),
    // TODO bitshift
}

#[derive(Debug)]
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

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

mod parse {
    use super::*;
    use crate::parse::{char_ws, fold_concat, tag_ws};

    fn or(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("||"), and, |acc, e| Expr::Or(acc.boxed(), e.boxed()))(input)
    }
    fn and(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("&&"), xor, |acc, e| {
            Expr::And(acc.boxed(), e.boxed())
        })(input)
    }
    fn xor(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("^"), add, |acc, e| Expr::Xor(acc.boxed(), e.boxed()))(input)
    }

    fn add(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("+"), sub, |acc, e| Expr::Add(acc.boxed(), e.boxed()))(input)
    }

    fn sub(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("-"), div, |acc, e| Expr::Sub(acc.boxed(), e.boxed()))(input)
    }

    fn div(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("/"), mul, |acc, e| Expr::Div(acc.boxed(), e.boxed()))(input)
    }

    fn mul(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("*"), preceding_sign, |acc, e| {
            Expr::Mul(acc.boxed(), e.boxed())
        })(input)
    }

    fn preceding_sign(i: &str) -> IResult<&str, Expr> {
        if let Ok((i, _)) = char_ws('-')(i) {
            let (i, e) = exponent(i)?;
            return Ok((i, Expr::Not(Box::new(e))));
        }

        value(i)
    }

    fn exponent(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("**"), value, |acc, e| {
            Expr::Exponent(acc.boxed(), e.boxed())
        })(input)
    }

    fn value(i: &str) -> IResult<&str, Expr> {
        // TODO Alternate Case of (nestings) here!!
        Value::parse(i).map(|(i, v)| (i, v.as_expr()))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_exponent() {
            assert!(parse::exponent("1**1**1").is_ok());
        }
    }
}
