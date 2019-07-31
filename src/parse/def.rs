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
use crate::parse::expression::Expr;

pub mod definition {
    pub struct Let {
        identifier: String,
        assign: Option<Box<crate::parse::expression::Expr>>,
    }

    pub struct Function {
        identifier: String,
        arguments: Vec<String>,
        body: super::FunctionBody,
    }
}

/// List of Let definitions, expressions, if/else pairs, for/whiles and return statements
pub struct FunctionBody {
    scope: Vec<definition::Let>,
    instructions: Vec<Instruction>,
}

/// Either an Expression, if/else pair, for/while loop or return statement
enum Instruction {
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
enum ForLoop {}
