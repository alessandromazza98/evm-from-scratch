use crate::{
    opcode::OpCode,
    utility::{add, div, mul, pop_x, push_x, sub},
};
use primitive_types::U256;

pub struct Evm {
    code: Box<[u8]>,
    stack: Vec<U256>,
}

impl Evm {
    pub fn new(code: Box<[u8]>, stack: Vec<U256>) -> Self {
        Self { code, stack }
    }

    pub fn execute(&mut self) -> ExecutionResult {
        let mut pc = 0;
        while pc < self.code.len() {
            if let Some(opcode) = OpCode::new(self.code[pc]) {
                if let ExecutionResult::Halt = self.transact(&mut pc, opcode) {
                    return ExecutionResult::Halt;
                }
            } else {
                return ExecutionResult::Revert;
            }
        }
        ExecutionResult::Success
    }

    pub fn transact(&mut self, pc: &mut usize, opcode: OpCode) -> ExecutionResult {
        match opcode {
            OpCode::Stop => ExecutionResult::Halt,
            OpCode::Push0 => {
                self.stack.push(0.into());
                *pc += 1;
                ExecutionResult::Success
            }
            OpCode::Push1 => push_x(1, pc, &mut self.stack, self.code.as_ref()),
            OpCode::Push2 => push_x(2, pc, &mut self.stack, self.code.as_ref()),
            OpCode::Push4 => push_x(4, pc, &mut self.stack, self.code.as_ref()),
            OpCode::Push6 => push_x(6, pc, &mut self.stack, self.code.as_ref()),
            OpCode::Push10 => push_x(10, pc, &mut self.stack, self.code.as_ref()),
            OpCode::Push11 => push_x(11, pc, &mut self.stack, self.code.as_ref()),
            OpCode::Push32 => push_x(32, pc, &mut self.stack, self.code.as_ref()),
            OpCode::Pop => pop_x(1, pc, &mut self.stack),
            OpCode::Add => add(pc, &mut self.stack),
            OpCode::Mul => mul(pc, &mut self.stack),
            OpCode::Sub => sub(pc, &mut self.stack),
            OpCode::Div => div(pc, &mut self.stack),
        }
    }

    /// Returns the stack at the end of execution. Note that the stack here is reversed.
    pub fn stack(&self) -> Vec<U256> {
        self.stack.iter().rev().cloned().collect()
    }
}

pub enum ExecutionResult {
    Success,
    Revert,
    Halt,
}
