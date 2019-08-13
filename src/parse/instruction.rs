use crate::parse::{char_ws, expression::Expr, for_loop::ForLoop, scope::*, tag_ws};
use nom::{
    branch::alt,
    combinator::opt,
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

#[cfg(test)]
mod function_body_tests {
    use super::FunctionBody;
    #[test]
    fn fn_body_1() {
        let input = "";

        assert!(FunctionBody::parse(input).is_ok());
    }
}

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
#[derive(Debug)]
pub struct FunctionBody {
    scope: Vec<Variable>,
    functions: Vec<Function>,
    instructions: Vec<Statement>,
}

impl FunctionBody {
    pub fn parse(input: &str) -> IResult<&str, FunctionBody> {
        enum FbItem {
            Var(Variable),
            Statement(Statement),
            Function(Function),
        }
        fn v_or_s(input: &str) -> IResult<&str, FbItem> {
            if let Ok((i, v)) = Variable::parse(input) {
                return Ok((i, FbItem::Var(v)));
            }

            if let Ok((i, f)) = Function::parse(input) {
                return Ok((i, FbItem::Function(f)));
            }

            let (i, s) = Statement::parse(input)?;

            Ok((i, FbItem::Statement(s)))
        }

        let (input, list) = many0(v_or_s)(input)?;
        let fb = list.into_iter().fold(
            FunctionBody {
                scope: Vec::new(),
                functions: Vec::new(),
                instructions: Vec::new(),
            },
            |mut acc, vs| {
                match vs {
                    FbItem::Var(v) => {
                        acc.scope.push(v);
                    }
                    FbItem::Statement(s) => {
                        acc.instructions.push(s);
                    }
                    FbItem::Function(f) => {
                        acc.functions.push(f);
                    }
                };

                acc
            },
        );

        Ok((input, fb))
    }
}

#[cfg(test)]
mod statement_tests {
    use super::Statement;

    #[test]
    fn test_return() {
        let tests = vec!["return", "   return  ", "return 1"];
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
        assert!(Statement::single_statement_body(" { return } ").is_ok());
    }

    #[test]
    fn test_while() {
        let input = "
            while (1) break
            ";
        assert!(Statement::parse_while(input).is_ok());
    }

    #[test]
    fn test_if() {
        let inputs = vec![
            "if (1) { return }",
            "\nif  \t( 1 )    break",
            "if(1){ return }",
        ];

        for input in inputs {
            assert!(Statement::parse_if_block(input).is_ok())
        }
    }
}

/// Either an Expression, if/else pair, for/while loop or return statement
/// Note that Mutations are expressions
#[derive(Debug)]
pub enum Statement {
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
    For(ForLoop),
    Break,
    Continue,
    Expression(Box<Expr>),
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
                        alt((
                            Statement::parse_break,
                            alt((Statement::parse_continue, Statement::parse_expression)),
                        )),
                    )),
                )),
            )),
        ))(input)
    }

    fn parse_for(input: &str) -> IResult<&str, Statement> {
        ForLoop::parse(input).map(|(i, f)| (i, Statement::For(f)))
    }

    fn parse_expression(input: &str) -> IResult<&str, Statement> {
        Expr::parse(input).map(|(i, e)| (i, Statement::Expression(Box::new(e))))
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
            functions: Vec::new(),
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
    pub fn single_statement_body(input: &str) -> IResult<&str, FunctionBody> {
        if let Ok((i, s)) = Statement::parse(input) {
            Ok((i, s.into_function_body()))
        } else {
            delimited(char_ws('{'), FunctionBody::parse, char_ws('}'))(input)
        }
    }
}
