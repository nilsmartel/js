use crate::vm::{
    instruction::{InstructionAddress, StackAddress},
    Instruction, Object,
};

const EMPTY_STACK: &'static str = "Expected Item on Stack";

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

    fn next(&mut self) {
        let instruction = self.instructions[self.currentFp].clone();
        self.currentFp += 1;

        use Instruction::*;
        match instruction {
            StoreGlobal(addr) => {
                let elem = self.stack.pop().expect(EMPTY_STACK);
                self.stack[addr as usize] = elem;
            }
            LoadGlobal(addr) => {
                self.stack.push(self.stack[addr as usize].clone());
            }
            Store(addr) => {
                let elem = self.stack.pop().expect(EMPTY_STACK);
                self.stack[(self.currentSp + addr) as usize] = elem;
            }

            _ => panic!("unimplemented Statement reached"),
        }
    }
}
