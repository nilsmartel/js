mod expression;
mod for_loop;
mod identifier;
mod instruction;
mod keywords;
mod obj;
mod scope;
mod util;

use util::*;

#[inline]
pub fn parse(source_code: &str) -> nom::IResult<&str, instruction::FunctionBody> {
    instruction::FunctionBody::parse(source_code)
}

#[cfg(test)]
mod toplevel_tests {
    use super::parse;
    use super::util;

    #[test]
    fn function() {
        let input = "
            function square(x) { return x*x }
        ";

        let result = dbg!(parse(input));

        assert!(result.is_ok());

        // Assert that only whitespace characters remain
        let (rest, _) = result.unwrap();
        let (left, _) = util::whitespace(rest).unwrap();
        assert_eq!("", left);
    }

    #[test]
    fn for_loop() {
        let input = "
            for (let i=0; i<12; i++) {
                if (i ** 2 && 123) {
                    continue
                } else {
                    break
                }
            }
        ";

        let result = dbg!(parse(input));

        assert!(result.is_ok());

        // Assert that only whitespace characters remain
        let (rest, _) = result.unwrap();
        let (left, _) = util::whitespace(rest).unwrap();
        assert_eq!("", left);
    }

    #[test]
    fn empty_body() {
        assert_eq!("", parse("{}").unwrap().0)
    }

    #[test]
    fn body() {
        let input = "
            function main() {
            window.setTimeout(onUpdate)
            }
        ";

        let result = dbg!(parse(input));

        assert!(result.is_ok());

        // Assert that only whitespace characters remain
        let (rest, _) = result.unwrap();
        let (left, _) = util::whitespace(rest).unwrap();
        assert_eq!("", left);
    }
}
