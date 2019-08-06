use crate::parse::expression::Expr;
use crate::parse::tag_ws;
use nom::bytes::complete::tag;
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
pub mod definition {
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
        body: super::FunctionBody,
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
}

/// List of Variable definitions, expressions, if/else pairs, for/whiles and return statements
pub struct FunctionBody {
    scope: Vec<definition::Variable>,
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
}

impl Statement {
    pub fn parse(input: &str) -> IResult<&str, Statement> {
        unimplemented!()
    }

    fn parse_if_else(input: &str) -> IResult<&str, Statement> {
        use nom::character::complete::char;
        use nom::sequence::delimited;
        let (input, _) = tag_ws("if")(input)?;
        let (input, condition) = delimited(char('('), Expr::parse, char(')'))(input)?;

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
        use nom::character::complete::char;
        use nom::sequence::delimited;
        if let Ok((i, s)) = Statement::parse(input) {
            Ok((i, s.into_function_body()))
        } else {
            delimited(char('{'), FunctionBody::parse, char('}'))(input)
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
        prerequisite: definition::Variable,
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
