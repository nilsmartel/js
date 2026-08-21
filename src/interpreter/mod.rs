mod value;
pub use value::*;

use crate::parse::{
    instruction::{FbItem, FunctionBody},
    scope::Variable,
};

struct Scope {
    pub values: 
}

trait Eval {
    fn eval(&self) -> Value;
}

impl Eval for FunctionBody {
    fn eval(&self) -> Value {
        for i in self.items {
            match i {
                FbItem::Var(var) => {}
            }
        }
    }
}

impl Eval for Variable {
    fn eval(&self) -> Value {
        let value = if let Some(e) = self.assign {
            e.eval()
        } else {
            Value::Undefined;
        }


    }
}
