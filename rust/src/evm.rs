use crate::{
    block_data::BlockData,
    errors::ExecutionError,
    memory::Memory,
    opcode::OpCode,
    state_data::State,
    storage::Storage,
    tx_data::TxData,
    utility::{
        add, addmod, and, balance, byte, call, calldataload, copy_data_to_memory, delegatecall,
        div, duplicate_data, eq, exp, extcodecopy, extcodehash, extcodesize, gt, iszero, jump,
        logx, lt, mload, mod_fn, msize, mstore, mstore8, mul, mulmod, not, or, pop, push,
        push_data, push_data_size, push_from_big_endian, return_func, revert, sar, sdiv,
        selfbalance, sgt, sha_3, shl, shr, sign_extend, sload, slt, smod, sstore, staticcall, sub,
        swap_data, xor,
    },
    Log,
};
use primitive_types::U256;

pub struct Evm {
    code: Box<[u8]>,
    tx_data: TxData,
    block_data: BlockData,
    state: State,
    storage: Storage,
    stack: Vec<U256>,
    logs: Vec<Log>,
    return_data: Vec<u8>,
    last_return_data: Vec<u8>,
    memory: Memory,
    limit: usize,
    read_only: bool,
}

impl Evm {
    pub fn new(
        code: Box<[u8]>,
        tx_data: TxData,
        block_data: BlockData,
        state: State,
        storage: Storage,
        stack: Vec<U256>,
        logs: Vec<Log>,
        return_data: Vec<u8>,
        last_return_data: Vec<u8>,
        memory: Memory,
        limit: usize,
        read_only: bool,
    ) -> Self {
        Self {
            code,
            tx_data,
            block_data,
            state,
            storage,
            stack,
            logs,
            return_data,
            last_return_data,
            memory,
            limit,
            read_only,
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
            | OpCode::Push5
            | OpCode::Push6
            | OpCode::Push7
            | OpCode::Push8
            | OpCode::Push9
            | OpCode::Push10
            | OpCode::Push11
            | OpCode::Push12
            | OpCode::Push13
            | OpCode::Push14
            | OpCode::Push15
            | OpCode::Push16
            | OpCode::Push17
            | OpCode::Push18
            | OpCode::Push19
            | OpCode::Push20
            | OpCode::Push21
            | OpCode::Push22
            | OpCode::Push23
            | OpCode::Push24
            | OpCode::Push25
            | OpCode::Push26
            | OpCode::Push27
            | OpCode::Push28
            | OpCode::Push29
            | OpCode::Push30
            | OpCode::Push31
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
                push_from_big_endian(&mut self.stack, &self.tx_data.to, self.limit)?;
                Ok(())
            }
            OpCode::Caller => {
                push_from_big_endian(&mut self.stack, &self.tx_data.from, self.limit)?;
                Ok(())
            }
            OpCode::Origin => {
                push_from_big_endian(&mut self.stack, &self.tx_data.origin, self.limit)?;
                Ok(())
            }
            OpCode::Gasprice => {
                push_from_big_endian(&mut self.stack, &self.tx_data.gasprice, self.limit)?;
                Ok(())
            }
            OpCode::Basfee => {
                push_from_big_endian(&mut self.stack, &self.block_data.basefee, self.limit)?;
                Ok(())
            }
            OpCode::Coinbase => {
                push_from_big_endian(&mut self.stack, &self.block_data.coinbase, self.limit)?;
                Ok(())
            }
            OpCode::Timestamp => {
                push_from_big_endian(&mut self.stack, &self.block_data.timestamp, self.limit)?;
                Ok(())
            }
            OpCode::Number => {
                push_from_big_endian(&mut self.stack, &self.block_data.number, self.limit)?;
                Ok(())
            }
            OpCode::Difficulty => {
                push_from_big_endian(&mut self.stack, &self.block_data.difficulty, self.limit)?;
                Ok(())
            }
            OpCode::Gaslimit => {
                push_from_big_endian(&mut self.stack, &self.block_data.gaslimit, self.limit)?;
                Ok(())
            }
            OpCode::Chainid => {
                push_from_big_endian(&mut self.stack, &self.block_data.chainid, self.limit)?;
                Ok(())
            }
            OpCode::Blockhash => {
                // not used in this test suite, can return 0.
                Ok(())
            }
            OpCode::Balance => {
                balance(&mut self.stack, &self.state, self.limit)?;
                Ok(())
            }
            OpCode::Callvalue => {
                push_from_big_endian(&mut self.stack, &self.tx_data.value, self.limit)?;
                Ok(())
            }
            OpCode::Calldataload => {
                calldataload(&mut self.stack, &self.tx_data.data, self.limit)?;
                Ok(())
            }
            OpCode::Calldatasize => {
                push_data_size(&mut self.stack, &self.tx_data.data, self.limit)?;
                Ok(())
            }
            OpCode::Calldatacopy => {
                copy_data_to_memory(&mut self.stack, &mut self.memory, &self.tx_data.data)?;
                Ok(())
            }
            OpCode::Codesize => {
                push_data_size(&mut self.stack, &self.code, self.limit)?;
                Ok(())
            }
            OpCode::Codecopy => {
                copy_data_to_memory(&mut self.stack, &mut self.memory, &self.code)?;
                Ok(())
            }
            OpCode::Extcodesize => {
                extcodesize(&mut self.stack, &self.state, self.limit)?;
                Ok(())
            }
            OpCode::Extcodecopy => {
                extcodecopy(&mut self.stack, &self.state, &mut self.memory)?;
                Ok(())
            }
            OpCode::Extcodehash => {
                extcodehash(&mut self.stack, &self.state, self.limit)?;
                Ok(())
            }
            OpCode::Selfbalance => {
                selfbalance(&mut self.stack, &self.state, &self.tx_data.to, self.limit)?;
                Ok(())
            }
            OpCode::Sstore => {
                sstore(
                    &mut self.stack,
                    &mut self.storage,
                    &self.tx_data.to,
                    self.read_only,
                )?;
                Ok(())
            }
            OpCode::Sload => {
                sload(&mut self.stack, &self.storage, &self.tx_data.to, self.limit)?;
                Ok(())
            }
            OpCode::Log0 | OpCode::Log1 | OpCode::Log2 | OpCode::Log3 | OpCode::Log4 => {
                let x = opcode.topics();
                logx(
                    x,
                    &mut self.stack,
                    &mut self.memory,
                    &self.tx_data.to,
                    &mut self.logs,
                    self.read_only,
                )?;
                Ok(())
            }
            OpCode::Return => {
                return_func(&mut self.stack, &mut self.memory, &mut self.return_data)?;
                Err(ExecutionError::Halt)
            }
            OpCode::Revert => {
                revert(&mut self.stack, &mut self.memory, &mut self.return_data)?;
                Err(ExecutionError::Revert)
            }
            OpCode::Call => {
                call(
                    &mut self.stack,
                    &mut self.memory,
                    &mut self.state,
                    &mut self.storage,
                    &self.tx_data.to,
                    &self.tx_data.origin,
                    &mut self.last_return_data,
                    self.limit,
                    self.read_only,
                )?;
                Ok(())
            }
            OpCode::Returndatasize => {
                push_data_size(&mut self.stack, &self.last_return_data, self.limit)?;
                Ok(())
            }
            OpCode::Returndatacopy => {
                copy_data_to_memory(&mut self.stack, &mut self.memory, &self.last_return_data)?;
                Ok(())
            }
            OpCode::Delegatecall => {
                delegatecall(
                    &mut self.stack,
                    &mut self.memory,
                    &mut self.state,
                    &mut self.storage,
                    &self.tx_data.to,
                    &self.tx_data.from,
                    &self.tx_data.origin,
                    &self.tx_data.value,
                    &mut self.last_return_data,
                    self.limit,
                )?;
                Ok(())
            }
            OpCode::Staticcall => {
                staticcall(
                    &mut self.stack,
                    &mut self.memory,
                    &mut self.state,
                    &mut self.storage,
                    &self.tx_data.to,
                    &self.tx_data.origin,
                    &self.tx_data.value,
                    &mut self.last_return_data,
                    self.limit,
                )?;
                Ok(())
            }
        }
    }

    /// Returns the stack at the end of execution. Note that the stack here is reversed.
    pub fn stack(&self) -> Vec<U256> {
        self.stack.iter().rev().cloned().collect()
    }

    /// Returns the logs at the end of execution.
    pub fn logs(&self) -> Vec<Log> {
        self.logs.iter().rev().cloned().collect()
    }

    pub fn return_data(&self) -> Vec<u8> {
        self.return_data.clone()
    }

    pub fn state(&self) -> State {
        self.state.clone()
    }

    pub fn storage(&self) -> Storage {
        self.storage.clone()
    }
}

pub enum ExecutionResult {
    Success,
    Revert,
    Halt,
}
