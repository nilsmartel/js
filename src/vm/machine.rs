use crate::vm::{
    instruction::{InstructionAddress, StackAddress},
    obj::{Arena, Object, Value},
    Instruction,
};

const EMPTY_STACK: &'static str = "Expected Item on Stack";

/// Virtual Stack Machine to interpret Instructions
pub struct VirtualMachine {
    arena: Arena<Object>,
    stack: Vec<Value>,
    instructions: Vec<Instruction>,
    currentFp: InstructionAddress,
    currentSp: StackAddress,
    functionPointerStack: Vec<InstructionAddress>,
    stackPointerStack: Vec<StackAddress>,
}

impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>) -> VirtualMachine {
        const INITIAL_STACK_SIZE: usize = 256;
        VirtualMachine {
            arena: Arena::new(),
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
                let addr = addr as usize;
                let elem = self.stack.pop().expect(EMPTY_STACK);
                self.stack[addr] = elem;
            }
            LoadGlobal(addr) => {
                let addr = addr as usize;
                self.stack.push(self.stack[addr].clone());
            }
            Store(addr) => {
                let elem = self.stack.pop().expect(EMPTY_STACK);
                self.stack[(self.currentSp + addr) as usize] = elem;
            }
            Load(addr) => {
                self.stack
                    .push(self.stack[(self.currentSp + addr) as usize].clone());
            }
            Get => {
                let key = self.stack.pop().expect(EMPTY_STACK);
                let obj = self.stack.pop().expect(EMPTY_STACK);

                self.stack.push(obj.get(key));
            }
            Add => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left.add(right, &mut self.arena));
            }
            op => unimplemented!("Operatio {:#?} not implemented", op),
        }
    }
}
