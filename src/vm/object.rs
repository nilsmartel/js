use crate::vm::instruction::InstructionAddress;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::rc::Rc;

pub type RcString = Rc<String>;

// TODO use GC
#[derive(Debug, Clone)]
struct Gc<T>(T);

/// Garbage Collected JavaScript Object
#[derive(Debug, Clone)]
pub enum Object {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(Rc<String>),
    Array(Gc<Vec<Object>>),
    Map(HashMap<RcString, Object>),
    Closure {
        enviroment: Gc<Vec<Object>>,
        function: InstructionAddress,
    },
}

impl Object {
    // TODO collect all prototype functions
    fn prototype_get(self, key: RcString) -> Object {
        use Object::*;
        match self {
            String(s) => match key.as_str() {
                "length" => return Number(s.len() as f64), // TODO Iterator etc.
                _ => Undefined,
            },
            Array(s) => match key.as_str() {
                "length" => return Number(s.0.len() as f64), // TODO Iterator etc.
                _ => Undefined,
            },
            _ => Undefined,
        }
    }
}

impl std::cmp::PartialEq for Object {
    fn eq(&self, o: &Object) -> bool {
        use Object::*;

        match (self, o) {
            (Undefined, Undefined) => true,
            (Null, Null) => true,
            (Boolean(a), Boolean(b)) => a == b,
            (Number(a), Number(b)) => a == b,
            (String(a), String(b)) => a == b,
            (Array(a), Array(b)) => {
                for (a, b) in a.0.iter().zip(b.0.iter()) {
                    if a != b {
                        return false;
                    }
                }

                true
            }
            // TODO map comparision?
            _ => false,
        }
    }
}

impl std::ops::BitAnd for Object {
    type Output = Object;
    fn bitand(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => Number((a as i64 & b as i64) as f64),
            _ => Undefined,
        }
    }
}

impl std::ops::Shr for Object {
    type Output = Object;
    fn shr(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => Number(((a as i64) >> b as i64) as f64),
            _ => Undefined,
        }
    }
}

impl std::ops::Shl for Object {
    type Output = Object;
    fn shl(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => Number(((a as i64) << b as i64) as f64),
            _ => Undefined,
        }
    }
}

impl std::ops::BitXor for Object {
    type Output = Object;
    fn bitxor(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => Number((a as i64 ^ b as i64) as f64),
            _ => Undefined,
        }
    }
}

impl std::ops::BitOr for Object {
    type Output = Object;
    fn bitor(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => Number((a as i64 | b as i64) as f64),
            _ => Undefined,
        }
    }
}

impl Sub for Object {
    type Output = Object;
    fn sub(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => Number(a + b),
            _ => Undefined,
        }
    }
}

impl std::ops::Rem for Object {
    type Output = Object;
    fn rem(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => {
                if b.abs() == 0.0 {
                    Undefined
                } else {
                    Number(a % b)
                }
            }
            _ => Undefined,
        }
    }
}

impl std::ops::Mul for Object {
    type Output = Object;
    fn mul(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => Number(a * b),
            _ => Undefined,
        }
    }
}

impl std::ops::Div for Object {
    type Output = Object;
    fn div(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (Number(a), Number(b)) => {
                if b.abs() == 0.0 {
                    Undefined
                } else {
                    Number(a / b)
                }
            }
            _ => Undefined,
        }
    }
}

impl Add for Object {
    type Output = Object;
    fn add(self, o: Object) -> Object {
        use Object::*;
        match (self, o) {
            (String(a), second) => {
                // unwrapping is safe, Upcastpcasting to string always works
                let b: RcString = second.upcast().unwrap();
                String(Rc::new(format!("{}{}", a, b)))
            }
            (first, String(b)) => {
                // unwrapping is safe, Upcastpcasting to string always works
                let a: RcString = first.upcast().unwrap();
                String(Rc::new(format!("{}{}", a, b)))
            }
            (Number(a), second) => {
                let b: Result<f64, ()> = second.upcast();

                if let Ok(b) = b {
                    Number(a + b)
                } else {
                    Undefined
                }
            }
            _ => Undefined,
        }
    }
}

impl std::ops::Not for Object {
    type Output = Object;
    fn not(self) -> Object {
        use Object::*;
        match self {
            Boolean(b) => Boolean(!b),
            _ => Undefined,
        }
    }
}

impl std::ops::Neg for Object {
    type Output = Object;
    fn neg(self) -> Object {
        use Object::*;
        match self {
            Number(n) => Number(-n),
            _ => Undefined,
        }
    }
}

impl Upcast<bool> for Object {
    fn upcast(self) -> Result<bool, ()> {
        use Object::*;
        match self {
            Boolean(b) => Ok(b),
            _ => Err(()),
        }
    }
}

impl Upcast<f64> for Object {
    fn upcast(self) -> Result<f64, ()> {
        use Object::*;
        match self {
            Number(n) => Ok(n),
            _ => Err(()),
        }
    }
}

impl Upcast<RcString> for Object {
    fn upcast(self) -> Result<RcString, ()> {
        use Object::*;
        match self {
            Undefined => Ok(Rc::new("undefined".to_string())),
            Null => Ok(Rc::new("null".to_string())),
            Boolean(b) => Ok(Rc::new(format!("{}", b))),
            Number(n) => Ok(Rc::new(format!("{}", n))),
            String(s) => Ok(s),
            Array(_) => Ok(Rc::new("[array]".to_string())),
            Map(_) => Ok(Rc::new("{object}".to_string())),
            Closure { .. } => Ok(Rc::new("function".to_string())),
        }
    }
}

trait Upcast<T> {
    fn upcast(self) -> Result<T, ()>;
}
