use crate::vm::Object;

/// Address in Variable Stack
pub type StackAddress = usize;
/// Address in Function Stack
pub type InstructionAddress = usize;

pub enum Instruction {
    StoreGlobal(StackAddress),
    LoadGlobal(StackAddress),
    Store(StackAddress), // Store relative to SP
    Load(StackAddress),  // Load relative to SP
    Push(Object),
    Get,                      // first.second or a['b'] or a[12]
    Call,                     // Calling Closures
    Jump(InstructionAddress), //
    JumpConditional,
    Add,
    Subtract,
    Mod,
    Div,
    Mul,
    And,
    Or,
    Equal,
    NotEqual,
    SmallerEqual,
    GreaterEqual,
    Not,
    Negation,
}
