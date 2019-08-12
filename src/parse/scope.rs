use crate::{parse::instruction::FunctionBody, parse::*};
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug)]
pub struct Variable {
    identifier: String,
    assign: Option<Box<crate::parse::expression::Expr>>,
}

impl Variable {
    pub fn parse(i: &str) -> IResult<&str, Variable> {
        let (i, _) = ignore_ws(tag("let"))(i)?;
        let (i, identifier) = ignore_ws(ident)(i)?;

        use nom::sequence::preceded;
        match preceded(ignore_ws(tag("=")), ignore_ws(expression::Expr::parse))(i) {
            Ok((rest, expr)) => Ok((
                rest,
                Variable {
                    identifier,
                    assign: Some(Box::new(expr)),
                },
            )),
            _ => Ok((
                i,
                Variable {
                    identifier,
                    assign: None,
                },
            )),
        }
    }
}

#[cfg(test)]
mod variable_test {
    use super::Variable;
    #[test]
    fn empty() {
        let input = "let x";
        assert!(Variable::parse(input).is_ok());
    }

    #[test]
    fn assign() {
        let input = "let xyz = 1 + 1 ";
        assert!(Variable::parse(input).is_ok());
    }
}

#[derive(Debug)]
pub struct Function {
    identifier: String,
    arguments: Vec<String>,
    body: FunctionBody,
}

impl Function {
    pub fn parse(input: &str) -> IResult<&str, Function> {
        use nom::sequence::{delimited, pair, preceded};

        let (input, (identifier, args)) = pair(
            preceded(tag_ws("function"), ignore_ws(ident)),
            delimited(
                char_ws('('),
                concat(char_ws(','), ignore_ws(ident)),
                char_ws(')'),
            ),
        )(input)?;

        let (input, body) = FunctionBody::parse(input)?;

        Ok((
            input,
            Function {
                identifier: identifier.into(),
                arguments: args,
                body,
            },
        ))
    }
}

#[cfg(test)]
mod function_test {
    use super::Function;
    #[test]
    fn function_test() {
        let input = "
            function one(x, y, z) {
                return 1
            }
            ";

        assert!(Function::parse(input).is_ok());
    }
}
