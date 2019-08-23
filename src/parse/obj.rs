use crate::parse::{
    char_ws, concat,
    expression::Expr,
    identifier::Identifier,
    ignore_ws,
    instruction::{FunctionBody, Statement},
    string_template::StringTemplate,
    tag_ws,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, none_of},
    combinator::map,
    multi::{many0, separated_list},
    sequence::{delimited, preceded, separated_pair},
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
    String(StringTemplate),
    Array(Vec<Expr>),
    Map(HashMap<Identifier, Expr>),
    Closure {
        args: Vec<Identifier>,
        body: FunctionBody,
    },
}

impl Object {
    pub fn parse(input: &str) -> IResult<&str, Object> {
        ignore_ws(alt((
            Object::parse_bool,
            Object::parse_number,
            Object::parse_string,
            Object::parse_array,
            Object::parse_map,
            Object::parse_closure,
        )))(input)
    }

    fn parse_bool(input: &str) -> IResult<&str, Object> {
        alt((
            map(tag("true"), |_| Object::Boolean(true)),
            map(tag("false"), |_| Object::Boolean(false)),
        ))(input)
    }

    fn parse_number(input: &str) -> IResult<&str, Object> {
        use nom::character::complete::{digit1, hex_digit1, oct_digit1};
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
                        map(digit1, |s: &str| s.parse::<u64>().unwrap()),
                    )),
                    |num: u64| num as f64,
                ),
            )),
            Object::Number,
        )(input)
    }

    fn parse_string(input: &str) -> IResult<&str, Object> {
        map(StringTemplate::parse, Object::String)(input)
    }

    fn parse_array(input: &str) -> IResult<&str, Object> {
        map(
            delimited(
                char('['),
                separated_list(char_ws(','), Expr::parse),
                char(']'),
            ),
            Object::Array,
        )(input)
    }

    fn parse_map(input: &str) -> IResult<&str, Object> {
        map(
            delimited(
                char_ws('{'),
                separated_list(
                    char_ws(','),
                    separated_pair(Identifier::parse_ws, char_ws(':'), Expr::parse),
                ),
                char_ws('}'),
            ),
            |pairs: Vec<(Identifier, Expr)>| {
                Object::Map(pairs.into_iter().collect::<HashMap<_, _>>())
            },
        )(input)
    }

    fn parse_closure(input: &str) -> IResult<&str, Object> {
        use nom::sequence::tuple;
        map(
            tuple((
                delimited(
                    char('('),
                    concat(char_ws(','), Identifier::parse_ws),
                    char_ws(')'),
                ),
                preceded(tag_ws("=>"), Statement::single_statement_body),
            )),
            |(args, body)| Object::Closure { args, body },
        )(input)
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
mod tests {
    use super::Object;

    #[test]
    fn parse_map() {
        let input = "{
            name: \"Steve\",
            position: {
                x: 1,
                y: 1
            }
        }";
        let result = dbg!(Object::parse_map(input));
        assert_eq!("", result.unwrap().0);
    }

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

    #[test]
    fn parse_closure() {
        assert!(Object::parse_closure("(a, b) => return").is_ok());
    }

    #[test]
    fn parse_empty_closure() {
        assert!(Object::parse_closure("() => return").is_ok());
    }
}
