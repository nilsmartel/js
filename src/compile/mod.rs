mod scope;
use crate::parse::{
    expression::*,
    for_loop::{ForLoop, ForLoopCondition},
    instruction::FunctionBody,
    scope::Function,
    Ast,
};
use crate::vm::Instruction;
use std::collections::HashMap;

pub fn generate_code(ast: Ast) -> Vec<Instruction> {
    let functions = get_all_functions(&ast, String::new());
}

fn get_all_functions<'a>(ast: &'a Ast, scope: String) -> HashMap<String, &'a Function> {
    ast.functions.iter().fold(
        map: HashMap::new(),
        |acc: HashMap<String, &'a Function>, f: &'a FunctionBody| {
            map[format!({}{}, &scope, f.identifier.0] = get_all_functions(:wq

        },
    )
}
