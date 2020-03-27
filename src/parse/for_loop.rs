use crate::parse::{
    char_ws,
    expression::Expr,
    instruction::{FunctionBody, Statement},
    scope::Variable,
    tag_ws,
};
use nom::{
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
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
#[derive(Debug)]
pub enum ForLoopCondition {
    // for(;;)
    CStyle {
        // This type of JavaScript only allows let as start of for loops
        // TODO Allow this to be optional
        prerequisite: Variable,
        condition: Box<Expr>,
        mutation: Box<Expr>,
    },

    // TODO implement
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
        ForLoopCondition::parse_c_style(input)
    }

    fn parse_c_style(input: &str) -> IResult<&str, ForLoopCondition> {
        let (rest, (prerequisite, (condition, mutation))) = separated_pair(
            Variable::parse,
            char_ws(';'),
            separated_pair(Expr::parse, char_ws(';'), Expr::parse),
        )(input)?;

        Ok((
            rest,
            ForLoopCondition::CStyle {
                prerequisite,
                condition: Box::new(condition),
                mutation: Box::new(mutation),
            },
        ))
    }
}

mod tests {
    use super::*;

    #[test]
    fn c_style_condition() {
        assert!(ForLoopCondition::parse_c_style("let x = 1; 1; 1").is_ok());
    }

    #[test]
    fn for_loop() {
        let cases = vec![
            "for (let i = 1; 1; 1) { return 1 }",
            "for (let i = 1; 1; 1) 1",
        ];

        for case in cases {
            assert!(ForLoop::parse(case).is_ok());
        }
    }
}
