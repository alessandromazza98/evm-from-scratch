use primitive_types::U256;

use crate::evm::ExecutionResult;

pub fn push_x(x: usize, pc: &mut usize, stack: &mut Vec<U256>, code: &[u8]) -> ExecutionResult {
    let start = *pc + 1;
    let end = start + x;

    if end <= code.len() {
        let push_data = &code[start..end];
        let value = U256::from_big_endian(push_data);

        stack.push(value);
        *pc += x + 1;

        ExecutionResult::Success
    } else {
        ExecutionResult::Revert
    }
}
