use crate::parse::{char_ws, fold_concat, not_followed, tag_ws};
use nom::{character::complete::char, IResult};

#[derive(Debug)]
pub struct Value(f64);
impl Value {
    pub fn parse(input: &str) -> IResult<&str, Value> {
        char_ws('1')(input).map(|(i, _)| (i, Value(1.0)))
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
    Equal(Box<Expr>, Box<Expr>),
    NotEqual(Box<Expr>, Box<Expr>),
    SmallerEq(Box<Expr>, Box<Expr>),
    GreaterEq(Box<Expr>, Box<Expr>),
    Smaller(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Exponent(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Neg(Box<Expr>),
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

impl Expr {
    pub fn parse(i: &str) -> IResult<&str, Expr> {
        Expr::or(i)
    }

    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn or(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("||"), Expr::and, |acc, e| {
            Expr::Or(acc.boxed(), e.boxed())
        })(input)
    }

    fn and(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("&&"), Expr::xor, |acc, e| {
            Expr::And(acc.boxed(), e.boxed())
        })(input)
    }

    fn xor(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("^"), Expr::equal, |acc, e| {
            Expr::Xor(acc.boxed(), e.boxed())
        })(input)
    }

    fn equal(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("=="), Expr::not_equal, |acc, e| {
            Expr::Equal(acc.boxed(), e.boxed())
        })(input)
    }

    fn not_equal(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("!="), Expr::greater_eq, |acc, e| {
            Expr::NotEqual(acc.boxed(), e.boxed())
        })(input)
    }

    fn greater_eq(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws(">="), Expr::smaller_eq, |acc, e| {
            Expr::GreaterEq(acc.boxed(), e.boxed())
        })(input)
    }

    fn smaller_eq(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("<="), Expr::greater, |acc, e| {
            Expr::SmallerEq(acc.boxed(), e.boxed())
        })(input)
    }

    fn greater(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws(">"), Expr::smaller, |acc, e| {
            Expr::Greater(acc.boxed(), e.boxed())
        })(input)
    }

    fn smaller(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("<"), Expr::add, |acc, e| {
            Expr::Smaller(acc.boxed(), e.boxed())
        })(input)
    }

    fn add(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("+"), Expr::sub, |acc, e| {
            Expr::Add(acc.boxed(), e.boxed())
        })(input)
    }

    fn sub(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("-"), Expr::div, |acc, e| {
            Expr::Sub(acc.boxed(), e.boxed())
        })(input)
    }

    fn div(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("/"), Expr::mul, |acc, e| {
            Expr::Div(acc.boxed(), e.boxed())
        })(input)
    }

    fn mul(input: &str) -> IResult<&str, Expr> {
        fold_concat(
            not_followed(char_ws('*'), char('*')),
            Expr::preceding_sign,
            |acc, e| Expr::Mul(acc.boxed(), e.boxed()),
        )(input)
    }

    fn preceding_sign(input: &str) -> IResult<&str, Expr> {
        if let Ok((input, _)) = char_ws('-')(input) {
            let (input, e) = Expr::exponent(input)?;
            return Ok((input, Expr::Neg(Box::new(e))));
        }

        if let Ok((input, _)) = char_ws('!')(input) {
            let (input, e) = Expr::exponent(input)?;
            return Ok((input, Expr::Not(Box::new(e))));
        }

        Expr::exponent(input)
    }

    fn exponent(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("**"), Expr::value, |acc, e| {
            Expr::Exponent(acc.boxed(), e.boxed())
        })(input)
    }

    fn value(i: &str) -> IResult<&str, Expr> {
        // TODO Alternate Case of (nestings) here!!
        Value::parse(i).map(|(i, v)| (i, v.as_expr()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expression_1() {
        assert!(Expr::parse("1").is_ok());
    }

    #[test]
    fn test_exponent() {
        assert!(Expr::exponent("1**1**1").is_ok());
    }
}
