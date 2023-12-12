use crate::{
    errors::ExecutionError,
    opcode::OpCode,
    utility::{
        add, addmod, div, exp, gt, lt, mod_fn, mul, mulmod, pop, push, push_data, sdiv,
        sign_extend, smod, sub, slt, sgt,
    },
};
use primitive_types::U256;

pub struct Evm {
    code: Box<[u8]>,
    stack: Vec<U256>,
    limit: usize,
}

impl Evm {
    pub fn new(code: Box<[u8]>, stack: Vec<U256>, limit: usize) -> Self {
        Self { code, stack, limit }
    }

    pub fn execute(&mut self) -> ExecutionResult {
        let mut pc = 0;
        while pc < self.code.len() {
            if let Some(opcode) = OpCode::new(self.code[pc]) {
                match self.transact(&mut pc, opcode) {
                    Ok(_) => {
                        // move the pc to the next instruction
                        pc += 1;
                    }
                    Err(ExecutionError::Halt) => return ExecutionResult::Halt,
                    Err(_) => return ExecutionResult::Revert,
                }
            } else {
                return ExecutionResult::Revert;
            }
        }
        ExecutionResult::Success
    }

    pub fn transact(&mut self, pc: &mut usize, opcode: OpCode) -> Result<(), ExecutionError> {
        match opcode {
            OpCode::Stop => Err(ExecutionError::Halt),
            OpCode::Push0 => {
                self.stack.push(0.into());
                Ok(())
            }
            OpCode::Push1
            | OpCode::Push2
            | OpCode::Push4
            | OpCode::Push6
            | OpCode::Push10
            | OpCode::Push11
            | OpCode::Push32 => {
                let start = *pc + 1;
                let push_data_size = opcode.push_data_size();
                let push_data = push_data(push_data_size, &self.code, start)?;
                push(&mut self.stack, push_data, self.limit)?;
                *pc += push_data_size;
                Ok(())
            }
            OpCode::Pop => {
                pop(&mut self.stack)?;
                Ok(())
            }
            OpCode::Add => {
                add(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Mul => {
                mul(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Sub => {
                sub(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Div => {
                div(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Mod => {
                mod_fn(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Addmod => {
                addmod(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Mulmod => {
                mulmod(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Exp => {
                exp(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Signextend => {
                sign_extend(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Sdiv => {
                sdiv(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Smod => {
                smod(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Lt => {
                lt(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Gt => {
                gt(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Slt => {
                slt(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Sgt => {
                sgt(&mut self.stack, self.limit)?;
                Ok(())
            }
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
