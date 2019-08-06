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
