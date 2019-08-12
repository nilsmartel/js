use crate::parse::*;
use nom::bytes::complete::tag;
use nom::IResult;

#[derive(Debug)]
pub struct Variable {
    identifier: String,
    assign: Option<Box<crate::parse::expression::Expr>>,
}

#[derive(Debug)]
pub struct Function {
    identifier: String,
    arguments: Vec<String>,
    body: crate::parse::instruction::FunctionBody,
}
// TODO implement Parse for Functions

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
