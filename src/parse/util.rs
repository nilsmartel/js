
use nom::IResult;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace() {
        assert_eq!(
            ("hello", ""),
            whitespace("hello")
            );

        assert_eq!(
            ("hello", "\n "),
            whitespace("\n hello")
        );

        assert_eq!(("", "    "), whitespace("    "));
        assert_eq!(("", ""), whitespace(""));
    }
}

pub fn whitespace(s: &str) -> (&str, &str) {
    nom::bytes::complete::take_while(|c| c == ' ' || c == '\n'|| c== '\r' || c == '\t')(s).unwrap()
}

#[inline]
pub fn ignore_ws<T>(f: impl Fn(&str) -> IResult<&str, T>) -> impl Fn(&str) -> IResult<&str, T> {
    move |i: &str| {
        let (i, _) = whitespace(i);
        f(i)
    }
}

pub fn ident(s: &str) -> IResult<&str, String> {
    nom::character::complete::alpha1(s).map(|(a, b)| (a, b.to_string()))
}
