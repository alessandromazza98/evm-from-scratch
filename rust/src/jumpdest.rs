use crate::{errors::ExecutionError, opcode::OpCode};
use bit_vec::BitVec;
use primitive_types::U256;

pub fn valid_jumpdest(position: U256, code: &[u8]) -> Result<bool, ExecutionError> {
    if let Some(opcode) = OpCode::new(code[position.as_usize()]) {
        if opcode != OpCode::Jumpdest {
            return Err(ExecutionError::NotValidJumpDestination);
        }
        let is_valid = is_code(position.as_usize(), code)?;
        Ok(is_valid)
    } else {
        Err(ExecutionError::InvalidOpcode)
    }
}

pub fn is_code(position: usize, code: &[u8]) -> Result<bool, ExecutionError> {
    let analysis = code_bitmap(code)?;
    let is_code = match analysis.get(position) {
        Some(value) => value,
        None => return Err(ExecutionError::NotValidJumpDestination),
    };
    Ok(is_code)
}

pub fn code_bitmap(code: &[u8]) -> Result<BitVec, ExecutionError> {
    // the bitmap is 4 bytes (32 bit) longer than necessary, in case the code ends with a PUSH32,
    // the algorithm will set bits on the bitvector outside the bounds of the actual code.
    let mut bitvec = BitVec::from_elem(code.len() + 32, true);
    let mut pc = 0;
    while pc < code.len() {
        if let Some(opcode) = OpCode::new(code[pc]) {
            if opcode.is_push() {
                let push_data_size = opcode.push_data_size();
                let start = pc;
                for i in start..=(start + push_data_size) {
                    bitvec.set(i, false);
                }
                pc += push_data_size;
            }
            pc += 1;
        } else {
            return Err(ExecutionError::InvalidOpcode);
        }
    }
    Ok(bitvec)
}
