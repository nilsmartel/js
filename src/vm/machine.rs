use crate::vm::{
    instruction::{InstructionAddress, StackAddress},
    obj::{Arena, Object},
    Instruction,
};

const EMPTY_STACK: &'static str = "Expected Item on Stack";

/// Virtual Stack Machine to interpret Instructions
pub struct VirtualMachine {
    arena: Arena<Object>,
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
                self.stack.push(left + right)
            }
            Sub => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left - right)
            }
            Rem => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left % right)
            }
            Div => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left / right)
            }
            Mul => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left * right)
            }

            BitwiseShiftLeft => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left << right)
            }
            BitwiseShiftRight => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left >> right)
            }
            BitwiseAnd => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left & right)
            }
            BitwiseOr => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left | right)
            }
            BitwiseXor => {
                let right = self.stack.pop().expect(EMPTY_STACK);
                let left = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(left ^ right)
            }
            BitwiseNot => {
                let elem = self.stack.pop().expect(EMPTY_STACK);
                self.stack.push(elem.bitwise_not())
            } // _ => panic!("unimplemented Statement reached"),
        }
    }
}
