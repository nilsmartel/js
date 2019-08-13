use crate::parse::{char_ws, fold_concat, not_followed, tag_ws};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

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
    Elvis {
        condition: Box<Expr>,
        case_true: Box<Expr>,
        case_false: Box<Expr>,
    },
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
    Mod(Box<Expr>, Box<Expr>), // Implement
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

impl MutationKind {
    fn parse(input: &str) -> IResult<&str, MutationKind> {
        use crate::parse::whitespace;
        use MutationKind::*;
        preceded(
            whitespace,
            alt((
                tag("="),
                tag("+="),
                tag("-="),
                tag("%="),
                tag("*="),
                tag("/="),
            )),
        )(input)
        .map(|(i, r)| {
            (
                i,
                match r {
                    "=" => Assign,
                    "+=" => AddAssign,
                    "-=" => SubtractAssign,
                    "%=" => ModAssign,
                    "*=" => MulAssign,
                    "/=" => DivAssign,
                    _ => unreachable!(),
                },
            )
        })
    }
}

impl Expr {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn parse(i: &str) -> IResult<&str, Expr> {
        Expr::elvis(i)
    }

    pub fn elvis(input: &str) -> IResult<&str, Expr> {
        let (input, expr) = Expr::or(input)?;

        if let Ok((input, (case_true, case_false))) = preceded(
            char_ws('?'),
            separated_pair(Expr::parse, char_ws(':'), Expr::parse),
        )(input)
        {
            return Ok((
                input,
                Expr::Elvis {
                    condition: expr.boxed(),
                    case_true: case_true.boxed(),
                    case_false: case_false.boxed(),
                },
            ));
        }

        Ok((input, expr))
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
            Expr::modulo,
            |acc, e| Expr::Mul(acc.boxed(), e.boxed()),
        )(input)
    }

    fn modulo(input: &str) -> IResult<&str, Expr> {
        fold_concat(tag_ws("%"), Expr::preceding_sign, |acc, e| {
            Expr::Mod(acc.boxed(), e.boxed())
        })(input)
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

    fn value(input: &str) -> IResult<&str, Expr> {
        alt((delimited(char_ws('('), Expr::parse, char_ws(')')), |i| {
            Value::parse(i).map(|(i, v)| (i, v.as_expr()))
        }))(input)
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
    fn expression_2() {
        assert!(Expr::parse("1 + 1 || 1 == 1 ^ 1 != 1/1 - 1").is_ok());
    }

    #[test]
    fn elvis() {
        assert!(Expr::elvis("1==1? 1+1 : 1-1").is_ok());
    }

    #[test]
    fn elvis_toplevel() {
        assert!(Expr::parse("1==1? 1+1 : 1-1").is_ok());
    }

    #[test]
    fn sign_1() {
        assert!(Expr::preceding_sign("-1").is_ok());
    }

    #[test]
    fn sign_2() {
        assert!(Expr::preceding_sign("!1").is_ok());
    }

    #[test]
    fn sign_toplevel() {
        assert!(Expr::parse("-1").is_ok());
    }

    #[test]
    fn nested_expressions() {
        assert!(Expr::parse("(1)").is_ok());
        assert!(Expr::parse("1*(1+1)").is_ok());
    }

    #[test]
    fn exponent() {
        assert!(Expr::exponent("1**1**1").is_ok());
    }
}
