use crate::vm::{
    instruction::{InstructionAddress, StackAddress},
    Instruction, Object,
};

/// Virtual Stack Machine to interpret Instructions
pub struct VirtualMachine {
    stack: Vec<Object>,
    instructions: Vec<Instruction>,
    currentFp: InstructionAddress,
    currentSp: StackAddress,
    functionPointerStack: Vec<InstructionAddress>,
    stackPointerStack: Vec<StackAddress>,
}

const INITIAL_STACK_SIZE: usize = 256;
impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>) -> VirtualMachine {
        VirtualMachine {
            stack: Vec::with_capacity(INITIAL_STACK_SIZE),
            instructions,
            currentFp: 0,
            currentSp: 0,
            functionPointerStack: Vec::new(),
            stackPointerStack: Vec::new(),
        }
    }
}
