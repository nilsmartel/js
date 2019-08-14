use crate::parse::{expression::Expr, scope::Function};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, none_of},
    combinator::map,
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

    fn parse_string(input_base: &str) -> IResult<&str, Object> {
        let (input, _) = char('"')(input_base)?;
        let input: &[u8] = input.as_bytes();

        let mut i = 0;
        let mut s = String::new();
        while input[i] != b'"' {
            let c = input[i] as char;

            match c {
                '\\' => {
                    i += 1;
                    s.push(input[i] as char);
                }
                _ => {
                    s.push(c);
                }
            }

            i += 1;
        }

        Ok((&input_base[input.len()..], Object::String(s)))
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
    fn parse_int() {
        assert!(Object::parse_number("123").is_ok());
    }

    #[test]
    fn parse_int_2() {
        assert!(Object::parse_number("0o123").is_ok());
    }

    #[test]
    fn parse_int_3() {
        assert!(Object::parse_number("0x123").is_ok());
    }

    #[test]
    fn parse_int_4() {
        assert!(Object::parse_number("0b101").is_ok());
    }

    #[test]
    fn parse_float() {
        assert!(Object::parse_number("3.14151").is_ok());
    }

    #[test]
    fn parse_empty_string() {
        assert!(Object::parse_string("\"\"").is_ok());
    }

    #[test]
    fn parse_string() {
        assert!(Object::parse_string("\"Hello World\"").is_ok());
    }
}
