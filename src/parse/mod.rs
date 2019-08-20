mod expression;
mod for_loop;
mod identifier;
mod instruction;
mod keywords;
mod obj;
mod scope;
mod util;

use util::*;

#[inline]
pub fn parse(source_code: &str) -> nom::IResult<&str, instruction::FunctionBody> {
    instruction::FunctionBody::parse(source_code)
}
