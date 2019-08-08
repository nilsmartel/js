use crate::parse::*;
use nom::bytes::complete::tag;
use nom::IResult;
pub struct Variable {
    identifier: String,
    assign: Option<Box<crate::parse::expression::Expr>>,
}

pub struct Function {
    identifier: String,
    arguments: Vec<String>,
    body: crate::parse::instruction::FunctionBody,
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
