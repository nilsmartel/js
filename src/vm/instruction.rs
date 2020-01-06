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
///
/// This also helps keeing track of the difference between Stack and Function Pointers internally
pub type StackAddress = isize;

/// Address in Function Stack
/// Should be an Unsigned Type, since only absolute addresses get referenced
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
    Sub,
    Rem,
    Div,
    Mul,
    And,
    Or,
    Equal,
    NotEqual,
    SmallerEqual,
    GreaterEqual,
    BitwiseShiftLeft,
    BitwiseShiftRight,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    Not,
    Negation,
}
