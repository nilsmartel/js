use nom::{IResult, Parser};

type Error<'a> = nom::error::Error<&'a str>;

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

    #[test]
    fn test_empty_concat() {
        let i = "";
        assert_eq!(concat(char_ws(','), char_ws('Q'))(i), Ok(("", vec![])));
    }
}

/// Remove all whitespace, newlines, tabs etc.
/// Will always suceed
pub fn whitespace(s: &str) -> IResult<&str, &str> {
    nom::bytes::complete::take_while(|c| c == ' ' || c == '\n' || c == '\r' || c == '\t').parse(s)
}

/// Wrap around a Parser to automatically ignore preceding whitespace
pub fn ignore_ws<'a, T>(
    mut f: impl Parser<&'a str, Output = T, Error = Error<'a>>,
) -> impl FnMut(&'a str) -> IResult<&'a str, T> {
    move |i: &'a str| {
        let (i, _) = whitespace(i).unwrap();
        f.parse(i)
    }
}

/// Tags a string while ignoring preceding whitespace
pub fn tag_ws<'a>(t: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    ignore_ws(move |input: &'a str| nom::bytes::complete::tag(t).parse(input))
}

pub fn not_followed<'a, A, B>(
    mut applied: impl Parser<&'a str, Output = A, Error = Error<'a>>,
    mut follow: impl Parser<&'a str, Output = B, Error = Error<'a>>,
) -> impl FnMut(&'a str) -> IResult<&'a str, A> {
    move |input: &'a str| {
        let (rest, result) = applied.parse(input)?;
        if follow.parse(rest).is_ok() {
            return Err(nom::Err::Error(nom::error::Error::new(
                rest,
                nom::error::ErrorKind::Not,
            )));
        }
        Ok((rest, result))
    }
}

/// Tags a character while ignoring preceding whitespace
pub fn char_ws<'a>(c: char) -> impl FnMut(&'a str) -> IResult<&'a str, char> {
    ignore_ws(move |input: &'a str| nom::character::complete::char(c).parse(input))
}

/// List of Elements, seperated by `sep` parser, might be empty
/// Note that this parser will fail, if the sep parser suceeds and the following element parser
/// fails.
/// If no element is parsed, an empty Array will be returned
pub fn concat<'a, T, Elem>(
    mut sep: impl Parser<&'a str, Output = T, Error = Error<'a>>,
    mut tag_elem: impl Parser<&'a str, Output = Elem, Error = Error<'a>>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<Elem>> {
    move |input: &'a str| {
        let mut v: Vec<Elem> = Vec::new();
        let first = tag_elem.parse(input);
        let (mut input, elem) = if let Ok(res) = first {
            res
        } else {
            return Ok((input, v));
        };

        v.push(elem);

        loop {
            if let Ok((i, _)) = sep.parse(input) {
                let (i, elem) = tag_elem.parse(i)?;
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
    sep: impl Parser<&'a str, Output = T, Error = Error<'a>>,
    tag_elem: impl Parser<&'a str, Output = E, Error = Error<'a>>,
    folding: impl Fn(E, E) -> E,
) -> impl FnMut(&'a str) -> IResult<&'a str, E> {
    let mut inner = concat(sep, tag_elem);
    move |input: &'a str| {
        let (rest, list) = inner(input)?;
        if list.len() == 0 {
            return Err(nom::Err::Error(nom::error::Error::new(
                rest,
                nom::error::ErrorKind::SeparatedList,
            )));
        }

        let mut iter = list.into_iter();
        let fst = iter.next().unwrap();
        Ok((rest, iter.fold(fst, &folding)))
    }
}
