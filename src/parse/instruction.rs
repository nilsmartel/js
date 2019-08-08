use crate::parse::char_ws;
use crate::parse::expression::Expr;
use crate::parse::scope;
use crate::parse::tag_ws;
use nom::combinator::opt;
use nom::sequence::{delimited, preceded};
use nom::IResult;

///
/// Definitions
///
/// The Scope of JavaScript may Include several Definitions
///
/// starting with
///
/// let <ident>;
/// let <ident> = <expr>;
/// function <ident> ( <list(',', <expr>)>) { ... }

/// List of Variable definitions, expressions, if/else pairs, for/whiles and return statements
pub struct FunctionBody {
    scope: Vec<scope::Variable>,
    instructions: Vec<Statement>,
}

impl FunctionBody {
    pub fn parse(input: &str) -> IResult<&str, FunctionBody> {
        tag_ws("<function body>")(input).map(|(rest, _): (&str, &str)| {
            (
                rest,
                FunctionBody {
                    scope: Vec::new(),
                    instructions: Vec::new(),
                },
            )
        })
    }
}

/// Either an Expression, if/else pair, for/while loop or return statement
/// Note that Mutations are expressions
enum Statement {
    Return(Option<Box<Expr>>),
    If {
        condition: Box<Expr>,
        body: FunctionBody,
        else_branch: Option<FunctionBody>,
    },
    While {
        condition: Box<Expr>,
        body: FunctionBody,
    },
    Break,
    Continue,
}

#[cfg(test)]
mod statement_tests {
    use super::Statement;

    #[test]
    fn test_return() {
        let tests = vec!["return", "   return  ", "return <expr>"];
        for test in tests {
            assert!(Statement::parse_return(test).is_ok());
        }
    }

    #[test]
    fn test_break_and_continue() {
        assert!(Statement::parse_break("  break").is_ok());
        assert!(Statement::parse_continue("\n  continue ").is_ok());
    }

    #[test]
    fn test_single_statement() {
        assert!(Statement::single_statement_body("return").is_ok());
        assert!(Statement::single_statement_body("  break").is_ok());
        assert!(Statement::single_statement_body(" { <function body> } ").is_ok());
    }

    #[test]
    fn test_while() {
        let input = "
            while (<expr>) break
            ";
        assert!(Statement::parse_while(input).is_ok());
    }

    #[test]
    fn test_if() {
        let inputs = vec![
            "if (<expr>) { <function body> }",
            "\nif  \t( <expr> )    break",
            "if(<expr>){<function body>}",
        ];

        for input in inputs {
            assert!(Statement::parse_if_block(input).is_ok())
        }
    }
}

impl Statement {
    pub fn parse(input: &str) -> IResult<&str, Statement> {
        use nom::branch::alt;
        alt((
            Statement::parse_if_block,
            alt((
                Statement::parse_return,
                alt((
                    Statement::parse_while,
                    alt((
                        Statement::parse_for,
                        alt((Statement::parse_break, Statement::parse_continue)),
                    )),
                )),
            )),
        ))(input)
    }

    fn parse_for(input: &str) -> IResult<&str, Statement> {
        // TODO implement
        Err(nom::Err::Error((input, nom::error::ErrorKind::Tag)))
    }

    fn parse_if_block(input: &str) -> IResult<&str, Statement> {
        let (input, condition) = preceded(
            tag_ws("if"),
            delimited(char_ws('('), Expr::parse, char_ws(')')),
        )(input)?;

        let (input, body) = Statement::single_statement_body(input)?;
        if let Ok((input, _)) = tag_ws("else")(input) {
            let (input, else_branch) = Statement::single_statement_body(input)?;
            return Ok((
                input,
                Statement::If {
                    condition: Box::new(condition),
                    body,
                    else_branch: Some(else_branch),
                },
            ));
        }

        Ok((
            input,
            Statement::If {
                condition: Box::new(condition),
                body,
                else_branch: None,
            },
        ))
    }

    fn parse_while(input: &str) -> IResult<&str, Statement> {
        let (input, condition) = preceded(
            tag_ws("while"),
            delimited(char_ws('('), Expr::parse, char_ws(')')),
        )(input)?;

        let (input, body) = Statement::single_statement_body(input)?;

        return Ok((
            input,
            Statement::While {
                condition: Box::new(condition),
                body,
            },
        ));
    }

    fn parse_return(input: &str) -> IResult<&str, Statement> {
        let (input, ret) = preceded(tag_ws("return"), opt(Expr::parse))(input)?;

        if let Some(expr) = ret {
            Ok((input, Statement::Return(Some(Box::new(expr)))))
        } else {
            Ok((input, Statement::Return(None)))
        }
    }

    fn parse_break(input: &str) -> IResult<&str, Statement> {
        tag_ws("break")(input).map(|(i, _)| (i, Statement::Break))
    }

    fn parse_continue(input: &str) -> IResult<&str, Statement> {
        tag_ws("continue")(input).map(|(i, _)| (i, Statement::Continue))
    }

    fn into_function_body(self) -> FunctionBody {
        FunctionBody {
            scope: Vec::new(),
            instructions: vec![self],
        }
    }

    /// May either be a single statement, or an functionbody wrapped in curly brackets
    /// ```js
    /// return something
    /// ```
    /// or
    /// ```js
    /// {
    ///     do_stuff()
    ///     do_more_stuff()
    /// }
    /// ```
    fn single_statement_body(input: &str) -> IResult<&str, FunctionBody> {
        if let Ok((i, s)) = Statement::parse(input) {
            Ok((i, s.into_function_body()))
        } else {
            delimited(char_ws('{'), FunctionBody::parse, char_ws('}'))(input)
        }
    }
}

/// Represents different kinds of for loops
/// e.g.
/// for (let i=0; i<len; i++) { ... }
/// for (let elem of array) { ... }
enum ForLoop {
    // for(;;)
    CStyle {
        // This type of JavaScript only allows let as start of for loops
        prerequisite: scope::Variable,
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
