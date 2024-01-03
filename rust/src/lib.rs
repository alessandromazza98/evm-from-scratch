mod errors;
mod evm;
mod jumpdest;
mod memory;
mod opcode;
pub mod tx_data;
mod utility;
use evm::{Evm, ExecutionResult};
use memory::Memory;
use primitive_types::U256;
use std::boxed::Box;
use tx_data::TxData;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>, _tx_to: impl AsRef<[u8]>) -> EvmResult {
    let code = _code.as_ref();
    let limit = 1024;
    let tx_data = TxData::new(_tx_to);

    let mut evm = Evm::new(Box::from(code), tx_data, Vec::new(), Memory::new(), limit);

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
