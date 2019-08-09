use crate::parse::{
    char_ws,
    expression::Expr,
    instruction::{FunctionBody, Statement},
    scope::Variable,
    tag_ws,
};
use nom::combinator::opt;
use nom::sequence::{delimited, preceded};
use nom::IResult;

pub struct ForLoop {
    condition: ForLoopCondition,
    body: FunctionBody,
}

impl ForLoop {
    pub fn parse(input: &str) -> IResult<&str, ForLoop> {
        let (input, condition) = preceded(
            tag_ws("for"),
            delimited(char_ws('('), ForLoopCondition::parse, char_ws(')')),
        )(input)?;

        let (input, body) = Statement::single_statement_body(input)?;

        Ok((input, ForLoop { condition, body }))
    }
}

/// Represents different kinds of for loop conditions
/// e.g.
/// ```js
/// for (let i=0; i<len; i++) { ... }
/// for (let elem of array) { ... }
/// ```
enum ForLoopCondition {
    // for(;;)
    CStyle {
        // This type of JavaScript only allows let as start of for loops
        prerequisite: Variable,
        condition: Box<Expr>,
        mutation: Box<Expr>,
    },
    // for(let x of y)
    ElemOfIter {
        element: String,
        iter: String,
    },
    // for(let x in y)
    KeyInIter {
        key: String,
        iter: String,
    },
}

impl ForLoopCondition {
    fn parse(input: &str) -> IResult<&str, ForLoopCondition> {
        Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)))
    }
}
