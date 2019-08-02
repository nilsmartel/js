pub struct Value(f64);

pub enum Expr {
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

use nom::IResult;
    use nom::bytes::complete::tag;
impl Expr {
    pub fn parse(i: &str) -> IResult<&str, Expr> {
        tag("<expr>")(i)
            .map(|(rest, _)| (rest, Expr::Value(Value(0.0))))
    }
}
