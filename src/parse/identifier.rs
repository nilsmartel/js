use crate::parse::keywords::is_keyword;
use crate::parse::*;
use nom::character::complete::alpha1;
use nom::IResult;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Hash)]
pub struct Identifier(String);

impl Identifier {
    /// Recognize Identifiers,
    /// Escapes keywords
    pub fn parse(input: &str) -> IResult<&str, Identifier> {
        let (rest, identifier) = alpha1(input)?;

        if is_keyword(identifier) {
            return Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)));
        }

        Ok((rest, Identifier(identifier.to_string())))
    }

    /// Recognize Identifiers,
    /// Escape keywords,
    /// Ignore Whitespace
    pub fn parse_ws(input: &str) -> IResult<&str, Identifier> {
        ignore_ws(Identifier::parse)(input)
    }
}
