use nom::IResult;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        assert_eq!(Ok(("hello", "")), whitespace("hello"));

        assert_eq!(Ok(("hello", "\n ")), whitespace("\n hello"));

        assert_eq!(Ok(("", "    ")), whitespace("    "));
        assert_eq!(Ok(("", "")), whitespace(""));
    }

    #[test]
    fn test_ignore_ws() {
        use nom::bytes::complete::tag;
        assert_eq!(Ok(("", "hello")), ignore_ws(tag("hello"))("hello"));
        assert_eq!(Ok(("", "hello")), ignore_ws(tag("hello"))("   hello"));
    }

    #[test]
    fn test_tag_ws() {
        assert_eq!(Ok(("", "hello")), tag_ws("hello")("hello"));
        assert_eq!(Ok(("", "hello")), tag_ws("hello")("   hello"));
    }
}

/// Remove all whitespace, newlines, tabs etc.
/// Will always suceed
pub fn whitespace(s: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(|c| c == ' ' || c == '\n' || c == '\r' || c == '\t')(s)
}

/// Wrap around a Parser to automatically ignore preceding whitespace
pub fn ignore_ws<'a, T>(
    f: impl Fn(&'a str) -> IResult<&'a str, T>,
) -> impl Fn(&'a str) -> IResult<&'a str, T> {
    move |i: &str| {
        let (i, _) = whitespace(i).unwrap();
        f(i)
    }
}

/// Tags a string while ignoring preceding whitespace
pub fn tag_ws<'a>(t: &'a str) -> impl Fn(&'a str) -> IResult<&'a str, &'a str> {
    ignore_ws(move |input: &str| nom::bytes::complete::tag(t)(input))
}

/// Tags a character while ignoring preceding whitespace
pub fn char_ws(c: char) -> impl Fn(&str) -> IResult<&str, char> {
    move |input: &str| ignore_ws(nom::character::complete::char(c))(input)
}

/// Recognize Identifiers,
/// Escapes keywords
pub fn ident(input: &str) -> IResult<&str, String> {
    use crate::parse::keywords::is_keyword;
    use nom::character::complete::alpha1;

    let (rest, identifier) = alpha1(input)?; //.map(|(a, b)| (a, b.to_string()))?

    if is_keyword(identifier) {
        return Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)));
    }

    Ok((rest, identifier.to_string()))
}

pub fn concat<R1, Elem>(
    sep: impl Fn(&str) -> IResult<&str, R1>,
    tag_elem: impl Fn(&str) -> IResult<&str, Elem>,
) -> IResult<&str, Vec<Elem>> {
    unimplemented!()
}
