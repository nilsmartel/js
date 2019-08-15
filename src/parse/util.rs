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

    #[test]
    fn test_concat() {
        let i = "Q,Q,Q,Q";
        assert_eq!(
            concat(char_ws(','), char_ws('Q'))(i),
            Ok(("", vec!['Q', 'Q', 'Q', 'Q']))
        );
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

pub fn not_followed<'a, A, B>(
    applied: impl Fn(&'a str) -> IResult<&'a str, A>,
    follow: impl Fn(&'a str) -> IResult<&'a str, B>,
) -> impl Fn(&'a str) -> IResult<&'a str, A> {
    move |input: &str| {
        let (rest, result) = applied(input)?;
        nom::combinator::not(&follow)(rest)?;
        Ok((rest, result))
    }
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

    let (rest, identifier) = dbg!(alpha1(input)?);

    if is_keyword(identifier) {
        return Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)));
    }

    Ok((rest, identifier.to_string()))
}

/// Recognize Identifiers,
/// Escape keywords,
/// Ignore Whitespace
pub fn ident_ws(input: &str) -> IResult<&str, String> {
    ignore_ws(ident)(input)
}

/// List of Elements, seperated by `sep` parser, might be empty
/// Note that this parser will fail, if the sep parser suceeds and the following element parser
/// fails
pub fn concat<'a, T, Elem>(
    sep: impl Fn(&'a str) -> IResult<&'a str, T>,
    tag_elem: impl Fn(&'a str) -> IResult<&'a str, Elem>,
) -> impl Fn(&'a str) -> IResult<&'a str, Vec<Elem>> {
    move |input: &str| {
        let mut v: Vec<Elem> = Vec::new();
        let (mut input, elem) = tag_elem(input)?;
        v.push(elem);

        loop {
            if let Ok((i, _)) = sep(input) {
                let (i, elem) = tag_elem(i)?;
                v.push(elem);
                input = i;
                continue;
            }
            break;
        }

        Ok((input, v))
    }
}

/// Fold a list of Elements, tagged be `tag_elem` and seperated by `sep`
pub fn fold_concat<'a, T, E>(
    sep: impl Fn(&'a str) -> IResult<&'a str, T>,
    tag_elem: impl Fn(&'a str) -> IResult<&'a str, E>,
    folding: impl Fn(E, E) -> E,
) -> impl Fn(&'a str) -> IResult<&'a str, E> {
    move |input: &str| {
        let (rest, list) = concat(&sep, &tag_elem)(input)?;
        if list.len() == 0 {
            return Err(nom::Err::Error((
                rest,
                nom::error::ErrorKind::SeparatedList,
            )));
        }

        let mut iter = list.into_iter();
        let fst = iter.next().unwrap();
        Ok((rest, iter.fold(fst, &folding)))
    }
}
