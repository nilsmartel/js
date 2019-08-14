use crate::parse::{expression::Expr, scope::Function};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, none_of},
    combinator::{map, not},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};
use std::collections::HashMap;

/// Represents Parsed JavaScript Object.
/// Note, that this is _not_ it's final representation,
/// just an Building Block
#[derive(Debug)]
pub enum Object {
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Expr>),
    Map(HashMap<String, Expr>),
    Closure(Function),
}

impl Object {
    pub fn parse(input: &str) -> IResult<&str, Object> {
        alt((Object::parse_bool, Object::parse_string))(input)
    }

    fn parse_bool(input: &str) -> IResult<&str, Object> {
        alt((
            map(tag("true"), |_| Object::Boolean(true)),
            map(tag("false"), |_| Object::Boolean(false)),
        ))(input)
    }

    fn parse_number(input: &str) -> IResult<&str, Object> {
        use nom::character::complete::{digit1, hex_digit1, oct_digit1, one_of};
        use nom::number::complete::double;
        // Integer parsing
        map(
            alt((
                double,
                map(
                    alt((
                        map(preceded(tag("0x"), hex_digit1), |s| {
                            u64::from_str_radix(s, 16).unwrap()
                        }),
                        map(preceded(tag("0o"), oct_digit1), |s| {
                            u64::from_str_radix(s, 8).unwrap()
                        }),
                        map(preceded(tag("0b"), bin_digit1), |s| {
                            u64::from_str_radix(&s, 2).unwrap()
                        }),
                    )),
                    |num: u64| num as f64,
                ),
            )),
            Object::Number,
        )(input)
    }

    fn parse_string(input: &str) -> IResult<&str, Object> {
        map(
            delimited(
                char('"'),
                map(
                    many0(alt((
                        preceded(
                            char('\\'),
                            alt((
                                map(char('n'), |_| '\n'),
                                map(char('t'), |_| '\t'),
                                map(char('\\'), |_| '\\'),
                                map(char('$'), |_| '$'),
                                map(char('r'), |_| '\r'),
                            )),
                        ),
                        none_of("\""),
                    ))),
                    |list| list.into_iter().collect::<String>(),
                ),
                char('"'),
            ),
            |s| Object::String(s),
        )(input)

        //delimited(tag("${"), Expr::parse, char('}')),
    }

    pub fn as_expr(self) -> Expr {
        Expr::Value(self)
    }
}

// TODO this is written poorly and unnessesarily allocates memory
fn bin_digit1(input: &str) -> IResult<&str, String> {
    use nom::{character::complete::one_of, multi::many1};
    map(many1(one_of("01")), |list| {
        list.into_iter().collect::<String>()
    })(input)
}

#[cfg(test)]
mod test_obj {
    use super::Object;

    #[test]
    fn parse_empty_string() {
        let input = "\"\"";
        assert!(Object::parse_string(input).is_ok())
    }

    #[test]
    fn parse_string() {
        let input = "\"Hello World\"";
        assert!(Object::parse_string(input).is_ok());
    }
}
