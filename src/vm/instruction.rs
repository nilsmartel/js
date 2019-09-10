use crate::vm::Object;

/// Address in Variable Stack
pub type StackAddress = usize;
/// Address in Function Stack
pub type InstructionAddress = usize;

#[derive(Debug, Clone)]
pub enum Instruction {
    StoreGlobal(StackAddress),
    LoadGlobal(StackAddress),
    Store(StackAddress), // Store relative to SP
    Load(StackAddress),  // Load relative to SP
    Push(Object),
    Get,                            // first.second or a['b'] or a[12]
    Jump,                           // Calling Closures
    JumpStatic(InstructionAddress), //
    JumpConditional(InstructionAddress),
    Add,
    Subtract,
    Rem,
    Div,
    Mul,
    And,          // impl
    Or,           // impl
    Equal,        // impl
    NotEqual,     // impl
    SmallerEqual, // imol
    GreaterEqual, // impl
    BitwiseShiftLeft,
    BitwiseShiftRight,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Not,
    Negation,
}
