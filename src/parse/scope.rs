use crate::{
    parse::*,
    parse::{expression::Expr, identifier::Identifier, instruction::FunctionBody},
};
use nom::IResult;

#[derive(Debug)]
pub struct Variable {
    pub identifier: Identifier,
    pub assign: Option<Box<Expr>>,
}

impl Variable {
    pub fn parse(i: &str) -> IResult<&str, Variable> {
        let (i, _) = tag_ws("let")(i)?;
        let (i, identifier) = Identifier::parse_ws(i)?;

        use nom::sequence::preceded;
        match preceded(tag_ws("="), ignore_ws(Expr::parse))(i) {
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
    pub identifier: Identifier,
    pub arguments: Vec<Identifier>,
    pub body: FunctionBody,
}

impl Function {
    pub fn parse(input: &str) -> IResult<&str, Function> {
        use nom::sequence::{delimited, pair, preceded};

        let (input, (identifier, arguments)) = pair(
            preceded(tag_ws("function"), Identifier::parse_ws),
            delimited(
                char_ws('('),
                concat(char_ws(','), Identifier::parse_ws),
                char_ws(')'),
            ),
        )(input)?;

        let (input, body) = delimited(char_ws('{'), FunctionBody::parse, char_ws('}'))(input)?;

        Ok((
            input,
            Function {
                identifier,
                arguments,
                body,
            },
        ))
    }
}

#[cfg(test)]
mod function_test {
    use super::Function;
    #[test]
    fn function_one() {
        let input = "
            function one(x, y, z) {
                return 1
            }
            ";

        let result = dbg!(Function::parse(input));
        assert!(result.is_ok());
    }

    #[test]
    fn function_square() {
        let input = "
            function square(x) {
                return x*x
            }";
        let result = dbg!(Function::parse(input));
        assert!(result.is_ok());
    }
}
