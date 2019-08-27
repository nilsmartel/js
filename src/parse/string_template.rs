use crate::parse::{char_ws, expression::Expr};
use nom::{
    character::complete::{char, none_of},
    combinator::map,
    multi::many0,
    sequence::delimited,
    IResult,
};

/// Template for String interpolation
#[derive(Debug)]
pub struct StringTemplate {
    start: String,
    end: Vec<(Expr, String)>,
}

impl StringTemplate {
    pub fn parse(input: &str) -> IResult<&str, StringTemplate> {
        map(
            delimited(char_ws('"'), many0(none_of("\"")), char('"')),
            |list: Vec<char>| StringTemplate {
                start: list.into_iter().collect(),
                end: Vec::new(),
            },
        )(input)
    }
}
