mod errors;
mod evm;
mod opcode;
mod utility;
use evm::{Evm, ExecutionResult};
use primitive_types::U256;
use std::boxed::Box;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let code = _code.as_ref();
    let limit = 1024;

    let mut evm = Evm::new(Box::from(code), Vec::new(), limit);

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
