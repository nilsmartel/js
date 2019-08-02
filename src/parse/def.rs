use crate::parse::expression::Expr;
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
use nom::IResult;

pub mod definition {
    use crate::parse::util::*;
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
            let (i, identifier) = ident(i)?;

            Ok((i, Let { identifier, assign: None }))
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
