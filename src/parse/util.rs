
use nom::IResult;

pub fn whitespace(s: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(|c| !(c == ' ' || c == '\n'|| c== '\r' || c == '\t'))(s)
}

pub fn ident(s: &str) -> IResult<&str, String> {
    nom::character::complete::alpha1(s).map(|(a, b)| (a, b.to_string()))
}
