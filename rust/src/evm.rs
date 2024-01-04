use crate::{
    block_data::BlockData,
    errors::ExecutionError,
    memory::Memory,
    opcode::OpCode,
    tx_data::TxData,
    utility::{
        add, addmod, address, and, basefee, byte, caller, coinbase, div, duplicate_data, eq, exp,
        gasprice, gt, iszero, jump, lt, mload, mod_fn, msize, mstore, mstore8, mul, mulmod, not,
        number, or, origin, pop, push, push_data, sar, sdiv, sgt, sha_3, shl, shr, sign_extend,
        slt, smod, sub, swap_data, timestamp, xor, difficulty,
    },
};
use primitive_types::U256;

pub struct Evm {
    code: Box<[u8]>,
    tx_data: TxData,
    block_data: BlockData,
    stack: Vec<U256>,
    memory: Memory,
    limit: usize,
}

impl Evm {
    pub fn new(
        code: Box<[u8]>,
        tx_data: TxData,
        block_data: BlockData,
        stack: Vec<U256>,
        memory: Memory,
        limit: usize,
    ) -> Self {
        Self {
            code,
            tx_data,
            block_data,
            stack,
            memory,
            limit,
        }
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
            | OpCode::Push3
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
            OpCode::Eq => {
                eq(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Iszero => {
                iszero(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Not => {
                not(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::And => {
                and(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Or => {
                or(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Xor => {
                xor(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Shl => {
                shl(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Shr => {
                shr(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Sar => {
                sar(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Byte => {
                byte(&mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Dup1
            | OpCode::Dup2
            | OpCode::Dup3
            | OpCode::Dup4
            | OpCode::Dup5
            | OpCode::Dup6
            | OpCode::Dup7
            | OpCode::Dup8
            | OpCode::Dup9
            | OpCode::Dup10
            | OpCode::Dup11
            | OpCode::Dup12
            | OpCode::Dup13
            | OpCode::Dup14
            | OpCode::Dup15
            | OpCode::Dup16 => {
                let duplicated_data_index = opcode.data_index();
                duplicate_data(duplicated_data_index, &mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Swap1
            | OpCode::Swap2
            | OpCode::Swap3
            | OpCode::Swap4
            | OpCode::Swap5
            | OpCode::Swap6
            | OpCode::Swap7
            | OpCode::Swap8
            | OpCode::Swap9
            | OpCode::Swap10
            | OpCode::Swap11
            | OpCode::Swap12
            | OpCode::Swap13
            | OpCode::Swap14
            | OpCode::Swap15
            | OpCode::Swap16 => {
                let swap_data_index = opcode.data_index();
                swap_data(swap_data_index, &mut self.stack, self.limit)?;
                Ok(())
            }
            OpCode::Pc => {
                push(&mut self.stack, (*pc).into(), self.limit)?;
                Ok(())
            }
            OpCode::Gas => {
                // it's not supported on this EVM. Always returns U256::MAX.
                push(&mut self.stack, U256::max_value(), self.limit)?;
                Ok(())
            }
            OpCode::Jump => {
                let counter = pop(&mut self.stack)?;
                jump(counter, &self.code, pc)?;
                Ok(())
            }
            OpCode::Jumpi => {
                let counter = pop(&mut self.stack)?;
                let b = pop(&mut self.stack)?;
                if b != 0.into() {
                    jump(counter, &self.code, pc)?;
                    Ok(())
                } else {
                    Ok(())
                }
            }
            OpCode::Jumpdest => {
                // do nothing.
                Ok(())
            }
            OpCode::Mstore => {
                mstore(&mut self.stack, &mut self.memory)?;
                Ok(())
            }
            OpCode::Mload => {
                mload(&mut self.stack, &mut self.memory, self.limit)?;
                Ok(())
            }
            OpCode::Mstore8 => {
                mstore8(&mut self.stack, &mut self.memory)?;
                Ok(())
            }
            OpCode::Msize => {
                msize(&mut self.stack, &mut self.memory, self.limit)?;
                Ok(())
            }
            OpCode::Sha3 => {
                sha_3(&mut self.stack, &mut self.memory, self.limit)?;
                Ok(())
            }
            OpCode::Address => {
                address(&mut self.stack, self.tx_data.to, self.limit)?;
                Ok(())
            }
            OpCode::Caller => {
                caller(&mut self.stack, self.tx_data.from, self.limit)?;
                Ok(())
            }
            OpCode::Origin => {
                origin(&mut self.stack, self.tx_data.origin, self.limit)?;
                Ok(())
            }
            OpCode::Gasprice => {
                gasprice(&mut self.stack, self.tx_data.gasprice, self.limit)?;
                Ok(())
            }
            OpCode::Basfee => {
                basefee(&mut self.stack, self.block_data.basefee, self.limit)?;
                Ok(())
            }
            OpCode::Coinbase => {
                coinbase(&mut self.stack, self.block_data.coinbase, self.limit)?;
                Ok(())
            }
            OpCode::Timestamp => {
                timestamp(&mut self.stack, self.block_data.timestamp, self.limit)?;
                Ok(())
            }
            OpCode::Number => {
                number(&mut self.stack, self.block_data.number, self.limit)?;
                Ok(())
            }
            OpCode::Difficulty => {
                difficulty(&mut self.stack, self.block_data.difficulty, self.limit)?;
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
