use crate::interpreter::register::Register;

use super::register::Register8Bit;

pub struct Stack {
    data: [u16; 16],
    stack_pointer: Register8Bit,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: [0; 16],
            stack_pointer: Register8Bit::new("SP"),
        }
    }

    pub fn push(&mut self, val: u16) {
        self.data[self.stack_pointer.value() as usize] = val;
        self.stack_pointer.increment_value_by(1);
    }

    pub fn pop(&mut self) -> u16 {
        self.data[self.stack_pointer.decrement_value_by(1) as usize]
    }

    pub fn reset(&mut self) {
        self.data.iter_mut().for_each(|m| *m = 0);
        self.stack_pointer.reset();
    }
}
