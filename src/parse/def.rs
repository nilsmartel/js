use crate::parse::expression::Expr;
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
    pub struct Let {
        identifier: String,
        assign: Option<Box<crate::parse::expression::Expr>>,
    }

    pub struct Function {
        identifier: String,
        arguments: Vec<String>,
        body: super::FunctionBody,
    }

    impl Let {
        pub fn parse(i: &str) -> IResult<&str, Let> {
            let (i, _) = ignore_ws(tag("let"))(i)?;
            let (i, identifier) = ignore_ws(ident)(i)?;

            use nom::sequence::preceded;
            match preceded(ignore_ws(tag("=")), ignore_ws(expression::Expr::parse))(i) {
                Ok((rest, expr)) => Ok((
                    rest,
                    Let {
                        identifier,
                        assign: Some(Box::new(expr)),
                    },
                )),
                _ => Ok((
                    i,
                    Let {
                        identifier,
                        assign: None,
                    },
                )),
            }
        }
    }
}

/// List of Let definitions, expressions, if/else pairs, for/whiles and return statements
pub struct FunctionBody {
    scope: Vec<definition::Let>,
    instructions: Vec<Statement>,
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
    pub fn parse(input: &str) -> IResult<&str, Statement> {}
}

/// Represents different kinds of for loops
/// e.g.
/// for (let i=0; i<len; i++) { ... }
/// for (let elem of array) { ... }
enum ForLoop {
    // for(;;)
    CStyle {
        // This type of JavaScript only allows let as start of for loops
        prerequisite: definition::Let,
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
