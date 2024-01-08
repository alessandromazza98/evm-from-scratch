mod block_data;
mod errors;
mod evm;
mod jumpdest;
mod memory;
mod opcode;
mod state_data;
mod tx_data;
mod utility;

use block_data::BlockData;
use evm::{Evm, ExecutionResult};
use memory::Memory;
use primitive_types::U256;
use state_data::State;
use std::{boxed::Box, collections::HashMap};
use tx_data::TxData;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(
    _code: impl AsRef<[u8]>,
    _tx_data: Vec<Vec<u8>>,
    _block_data: Vec<Vec<u8>>,
    _state_data: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>,
) -> EvmResult {
    let code = _code.as_ref();
    let limit = 1024;
    let tx_data = TxData::new(_tx_data);
    let block_data = BlockData::new(_block_data);
    let state_data = State::new(_state_data);

    let mut evm = Evm::new(
        Box::from(code),
        tx_data,
        block_data,
        state_data,
        Vec::new(),
        Memory::new(),
        limit,
    );

    let result = evm.execute();

    match result {
        ExecutionResult::Success => EvmResult {
            stack: evm.stack(),
            success: true,
        },
        ExecutionResult::Revert => EvmResult {
            stack: evm.stack(),
            success: false,
        },
        ExecutionResult::Halt => EvmResult {
            stack: evm.stack(),
            success: true,
        },
    }
}
