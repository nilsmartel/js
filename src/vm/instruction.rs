use crate::vm::Object;

/// Address in Variable Stack
/// It's an isize instead of an usize.
/// This goes against intuition, but this way one can use negatives
/// in order to describe the operation
/// ```
/// let b = 7;
/// a - b
/// ```
/// as
/// ```
/// let b = -7;
/// a + b
/// ```
pub type StackAddress = isize;
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
