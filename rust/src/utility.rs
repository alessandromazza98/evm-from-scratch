use crate::{
    block_data::BlockData,
    errors::ExecutionError,
    evm::{Evm, ExecutionResult},
    jumpdest::valid_jumpdest,
    memory::Memory,
    state_data::State,
    storage::Storage,
    tx_data::TxData,
    Log,
};
use primitive_types::U256;
use sha3::{Digest, Keccak256};

pub fn push_data(push_data_size: usize, code: &[u8], start: usize) -> Result<U256, ExecutionError> {
    let remaining_code = &code[start..];
    if remaining_code.len() < push_data_size {
        Err(ExecutionError::InsufficientCodeItems)
    } else {
        let push_data = &remaining_code[..push_data_size];
        let push_data = U256::from_big_endian(push_data);
        Ok(push_data)
    }
}

pub fn push(stack: &mut Vec<U256>, item: U256, limit: usize) -> Result<(), ExecutionError> {
    if stack.len() + 1 > limit {
        Err(ExecutionError::StackOverflow)
    } else {
        stack.push(item);
        Ok(())
    }
}

pub fn pop(stack: &mut Vec<U256>) -> Result<U256, ExecutionError> {
    if stack.is_empty() {
        Err(ExecutionError::StackUnderflow)
    } else {
        let item = stack
            .pop()
            .expect("the stack should have at least one item!");
        Ok(item)
    }
}

pub fn add(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    // we don't care if it woverflows. EVM simply wraps around.
    let (sum, _) = a.overflowing_add(b);
    push(stack, sum, limit)?;

    Ok(sum)
}

pub fn mul(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    // we don't care if it overflows. EVM simply wraps around.
    let (mul, _) = a.overflowing_mul(b);
    push(stack, mul, limit)?;

    Ok(mul)
}

pub fn sub(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    // we don't care if it overflows. EVM simply wraps around.
    let (sub, _) = a.overflowing_sub(b);
    push(stack, sub, limit)?;

    Ok(sub)
}

pub fn div(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let div = a.checked_div(b).unwrap_or(0.into());
    push(stack, div, limit)?;
    Ok(div)
}

pub fn mod_fn(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let result = a.checked_rem(b).unwrap_or(0.into());
    push(stack, result, limit)?;
    Ok(result)
}

pub fn addmod(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let _ = add(stack, limit)?;
    mod_fn(stack, limit)
}

pub fn mulmod(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;
    let n = pop(stack)?;

    let mul = a.full_mul(b);
    match mul.checked_rem(n.into()) {
        Some(result) => {
            let result = result.try_into().unwrap_or(0.into());
            push(stack, result, limit)?;
            Ok(result)
        }
        None => {
            let result = 0.into();
            push(stack, result, limit)?;
            Ok(result)
        }
    }
}

pub fn exp(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let exponent = pop(stack)?;

    let (result, _) = a.overflowing_pow(exponent);
    push(stack, result, limit)?;
    Ok(result)
}

pub fn sign_extend(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let b = pop(stack)?;
    let x = pop(stack)?;

    let sign_byte = x.byte(b.as_usize());

    // convert U256 to a little-endian byte array
    let mut data = [0u8; 32];
    x.to_little_endian(&mut data);

    for i in 0..32 {
        if i as usize > b.as_usize() {
            if sign_byte > 0x7f {
                data[i] = 0xFF;
            } else {
                data[i] = 0x00;
            }
        }
    }

    // convert the modified byte array back to U256
    let result = U256::from_little_endian(&data);

    push(stack, result, limit)?;
    Ok(result)
}

pub fn sdiv(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let mut a = pop(stack)?;
    let mut b = pop(stack)?;

    let is_a_negative = a.bit(255);
    let is_b_negative = b.bit(255);

    if is_a_negative {
        (a, _) = a.overflowing_neg();
    }

    if is_b_negative {
        (b, _) = b.overflowing_neg();
    }

    let mut result = a.checked_div(b).unwrap_or(0.into());

    if is_a_negative != is_b_negative {
        (result, _) = result.overflowing_neg();
    }

    push(stack, result, limit)?;
    Ok(result)
}

pub fn smod(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let mut a = pop(stack)?;
    let mut b = pop(stack)?;

    let is_a_negative = a.bit(255);
    let is_b_negative = b.bit(255);

    if is_a_negative {
        (a, _) = a.overflowing_neg();
    }

    if is_b_negative {
        (b, _) = b.overflowing_neg();
    }

    let mut result = a.checked_rem(b).unwrap_or(0.into());

    if is_a_negative && is_b_negative {
        (result, _) = result.overflowing_neg();
    }

    push(stack, result, limit)?;
    Ok(result)
}

pub fn lt(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let result = a < b;
    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    push(stack, result, limit)?;
    Ok(result)
}

pub fn gt(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let result = a > b;
    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    push(stack, result, limit)?;
    Ok(result)
}

pub fn slt(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let is_a_negative = a.bit(255);
    let is_b_negative = b.bit(255);

    let result = match (is_a_negative, is_b_negative) {
        (true, true) => !(a.overflowing_neg() <= b.overflowing_neg()),
        (true, false) => true,
        (false, true) => false,
        (false, false) => a < b,
    };

    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    push(stack, result, limit)?;
    Ok(result)
}

pub fn sgt(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let is_a_negative = a.bit(255);
    let is_b_negative = b.bit(255);

    let result = match (is_a_negative, is_b_negative) {
        (true, true) => !(a.overflowing_neg() >= b.overflowing_neg()),
        (true, false) => false,
        (false, true) => true,
        (false, false) => a > b,
    };

    let result = match result {
        true => 1.into(),
        false => 0.into(),
    };

    push(stack, result, limit)?;
    Ok(result)
}

pub fn eq(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let result = if a == b { 1.into() } else { 0.into() };

    push(stack, result, limit)?;
    Ok(result)
}

pub fn iszero(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;

    let result = if a.is_zero() { 1.into() } else { 0.into() };

    push(stack, result, limit)?;
    Ok(result)
}

pub fn not(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let a_negated = !a;

    push(stack, a_negated, limit)?;
    Ok(a_negated)
}

pub fn and(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let result = a & b;

    push(stack, result, limit)?;
    Ok(result)
}

pub fn or(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let result = a | b;

    push(stack, result, limit)?;
    Ok(result)
}

pub fn xor(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    let result = a ^ b;

    push(stack, result, limit)?;
    Ok(result)
}

pub fn shl(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let shift = pop(stack)?;
    let value = pop(stack)?;

    let result = value << shift;

    push(stack, result, limit)?;
    Ok(result)
}

pub fn shr(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let shift = pop(stack)?;
    let value = pop(stack)?;

    let result = value >> shift;

    push(stack, result, limit)?;
    Ok(result)
}

pub fn sar(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let shift = pop(stack)?;
    let value = pop(stack)?;

    let is_value_negative = value.bit(255);

    let mut result: U256;

    if is_value_negative {
        let (value_negated, _) = value.overflowing_neg();
        result = value_negated >> shift;
        if result.is_zero() {
            result = U256::max_value();
        } else {
            result = result.overflowing_neg().0;
        }
    } else {
        result = value >> shift;
    }

    push(stack, result, limit)?;
    Ok(result)
}

pub fn byte(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let i = pop(stack)?;
    let x = pop(stack)?;

    // `i` must be less than 31 to avoid exceeding the max byte width (32).
    if i > 31.into() {
        // if the byte offset is out of range, the result is 0.
        push(stack, 0.into(), limit)?;
        return Ok(0.into());
    }

    // `31 - i` is needed because in the `byte` opcode `i` represents the byte offset starting from the most significant byte.
    let x_byte = x.byte(31 - i.as_usize());
    let x_byte = x_byte.into();

    push(stack, x_byte, limit)?;
    Ok(x_byte)
}

pub fn duplicate_data(
    duplicated_data_index: usize,
    stack: &mut Vec<U256>,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let mut ignored_values = vec![];
    // pop all preceding values from the stack.
    for _ in 0..duplicated_data_index - 1 {
        ignored_values.push(pop(stack)?);
    }

    let duplicated_data = pop(stack)?;

    // re-push original (duplicated) data into the stack
    push(stack, duplicated_data, limit)?;

    // re-push ignored data into the stack.
    for ignored_value in ignored_values.into_iter().rev() {
        push(stack, ignored_value, limit)?;
    }

    // push the duplicated value into the stack.
    push(stack, duplicated_data, limit)?;

    Ok(duplicated_data)
}

pub fn swap_data(
    swap_data_index: usize,
    stack: &mut Vec<U256>,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let first_item = pop(stack)?;

    let mut ignored_values = vec![];
    // pop all preceding values from the stack.
    for _ in 0..swap_data_index - 1 {
        ignored_values.push(pop(stack)?);
    }

    let swap_data = pop(stack)?;

    // push first item into the stack
    push(stack, first_item, limit)?;

    // re-push ignored data into the stack.
    for ignored_value in ignored_values.into_iter().rev() {
        push(stack, ignored_value, limit)?;
    }

    // push the swap data into the stack.
    push(stack, swap_data, limit)?;

    Ok(swap_data)
}

pub fn jump(counter: U256, code: &[u8], pc: &mut usize) -> Result<U256, ExecutionError> {
    let is_valid = valid_jumpdest(counter, code)?;
    if is_valid {
        *pc = counter.as_usize();
        Ok(counter)
    } else {
        Err(ExecutionError::NotValidJumpDestination)
    }
}

pub fn mstore(stack: &mut Vec<U256>, memory: &mut Memory) -> Result<U256, ExecutionError> {
    let offset = pop(stack)?.as_usize();
    let value = pop(stack)?;

    memory.save_word(offset, value)
}

pub fn mload(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let offset = pop(stack)?.as_usize();
    let value = memory.get_word(offset)?;

    push(stack, value, limit)?;
    Ok(value.into())
}

pub fn mstore8(stack: &mut Vec<U256>, memory: &mut Memory) -> Result<U256, ExecutionError> {
    let offset = pop(stack)?.as_usize();
    let value = pop(stack)?;

    let mut value_bytes = [0u8; 32];
    value.to_big_endian(&mut value_bytes);

    memory.save_byte(offset, value_bytes[31])?;
    Ok(value)
}

pub fn msize(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let size = memory.store.len().into();

    push(stack, size, limit)?;
    Ok(size)
}

pub fn sha_3(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let offset = pop(stack)?.as_usize();
    let size = pop(stack)?.as_usize();

    let value = &memory.store[offset..(offset + size)];

    let result = U256::from_big_endian(&sha3_hash(&value));

    push(stack, result, limit)?;

    Ok(result)
}

pub fn sha3_hash(data: &[u8]) -> [u8; 32] {
    if data.is_empty() {
        let result = [0; 32];
        result
    } else {
        // create hash
        let mut hasher = Keccak256::new();
        hasher.update(data);
        let result = hasher.finalize();

        result.into()
    }
}

pub fn push_from_big_endian(
    stack: &mut Vec<U256>,
    slice: &[u8],
    limit: usize,
) -> Result<U256, ExecutionError> {
    let value = U256::from_big_endian(slice);

    push(stack, value, limit)?;
    Ok(value)
}

pub fn balance(stack: &mut Vec<U256>, state: &State, limit: usize) -> Result<U256, ExecutionError> {
    let address = pop(stack)?;
    let balance = state.get_balance(address);
    let mut balance_bytes = [0u8; 32];
    balance.to_big_endian(&mut balance_bytes);

    push_from_big_endian(stack, &balance_bytes, limit)
}

pub fn calldataload(
    stack: &mut Vec<U256>,
    data: &Vec<u8>,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let index = pop(stack)?.as_usize();

    const VALUE_NUM_BYTES: usize = 32;
    let mut copied_data = [0u8; VALUE_NUM_BYTES];

    // check if offset is within bounds of data
    if index < data.len() {
        // calculate the amount of data available to copy
        let available_data = &data[index..];

        // calculate the actual copy size based on available data
        let copy_size = std::cmp::min(VALUE_NUM_BYTES, available_data.len());

        // copy available data to the destination
        copied_data[..copy_size].copy_from_slice(&available_data[..copy_size]);
    }

    let value = &copied_data;

    push_from_big_endian(stack, value, limit)
}

pub fn copy_data_to_memory(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    data: &[u8],
) -> Result<(), ExecutionError> {
    let dest_offset = pop(stack)?.as_usize();
    let offset = pop(stack)?.as_usize();
    let size = pop(stack)?.as_usize();

    let mut copied_data = vec![0; size];

    // check if offset is within bounds of data
    if offset < data.len() {
        // calculate the amount of data available to copy
        let available_data = &data[offset..];

        // calculate the actual copy size based on available data
        let copy_size = std::cmp::min(size, available_data.len());

        // copy available data to the destination
        copied_data[..copy_size].copy_from_slice(&available_data[..copy_size]);
    }

    for (i, byte) in copied_data.iter().enumerate() {
        memory.save_byte(dest_offset + i, *byte)?;
    }

    Ok(())
}

pub fn push_data_size(
    stack: &mut Vec<U256>,
    data: &[u8],
    limit: usize,
) -> Result<U256, ExecutionError> {
    let size = data.len().into();

    push(stack, size, limit)?;
    Ok(size)
}

pub fn extcodesize(
    stack: &mut Vec<U256>,
    state: &State,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let address = pop(stack)?;
    let code = state.get_code(address);

    push_data_size(stack, &code, limit)
}

pub fn extcodecopy(
    stack: &mut Vec<U256>,
    state: &State,
    memory: &mut Memory,
) -> Result<(), ExecutionError> {
    let address = pop(stack)?;
    let code = state.get_code(address);

    copy_data_to_memory(stack, memory, &code)
}

pub fn extcodehash(
    stack: &mut Vec<U256>,
    state: &State,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let address = pop(stack)?;
    let code = state.get_code(address);

    let result = sha3_hash(&code).into();

    push(stack, result, limit)?;
    Ok(result)
}

pub fn selfbalance(
    stack: &mut Vec<U256>,
    state: &State,
    address: &[u8],
    limit: usize,
) -> Result<U256, ExecutionError> {
    let balance = state.get_balance(address.into());
    let mut balance_bytes = [0u8; 32];
    balance.to_big_endian(&mut balance_bytes);

    push_from_big_endian(stack, &balance_bytes, limit)
}

pub fn sstore(
    stack: &mut Vec<U256>,
    storage: &mut Storage,
    address: &[u8],
    read_only: bool,
) -> Result<U256, ExecutionError> {
    if read_only {
        return Err(ExecutionError::ReadOnly);
    }
    let key = pop(stack)?;
    let value = pop(stack)?;

    storage.set_word(U256::from_big_endian(address), key, value);
    Ok(value)
}

pub fn sload(
    stack: &mut Vec<U256>,
    storage: &Storage,
    address: &[u8],
    limit: usize,
) -> Result<U256, ExecutionError> {
    let key = pop(stack)?;

    let value = storage.load_word(U256::from_big_endian(address), key);
    push(stack, value, limit)?;
    Ok(value)
}

pub fn add_log(log: Log, logs: &mut Vec<Log>) -> Result<(), ExecutionError> {
    logs.push(log);
    Ok(())
}

pub fn logx(
    x: usize,
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    address: &[u8],
    logs: &mut Vec<Log>,
    read_only: bool,
) -> Result<(), ExecutionError> {
    if read_only {
        return Err(ExecutionError::ReadOnly);
    }
    let offset = pop(stack)?.as_usize();
    let size = pop(stack)?.as_usize();
    let mut topics = vec![];

    for _ in 0..x {
        let topic = pop(stack)?;
        topics.push(topic);
    }

    let data = memory.get_bytes(offset, size)?;

    let log = Log::new(U256::from_big_endian(address), data, topics);
    add_log(log, logs)?;

    Ok(())
}

pub fn return_func(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    return_data: &mut Vec<u8>,
) -> Result<(), ExecutionError> {
    let offset = pop(stack)?.as_usize();
    let size = pop(stack)?.as_usize();

    let data = memory.get_bytes(offset, size)?;
    *return_data = data;

    Ok(())
}

pub fn revert(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    return_data: &mut Vec<u8>,
) -> Result<(), ExecutionError> {
    return_func(stack, memory, return_data)?;
    stack.clear();

    Ok(())
}

pub fn call(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    state: &mut State,
    storage: &mut Storage,
    tx_to: &[u8],
    tx_origin: &[u8],
    last_ret_data: &mut Vec<u8>,
    limit: usize,
    read_only: bool,
) -> Result<U256, ExecutionError> {
    let _gas = pop(stack)?;
    let address = pop(stack)?;
    let value = pop(stack)?;

    if read_only && !value.is_zero() {
        return Err(ExecutionError::ReadOnly);
    }

    let args_offset = pop(stack)?.as_usize();
    let args_size = pop(stack)?.as_usize();
    let ret_offset = pop(stack)?.as_usize();
    let _ret_size = pop(stack)?.as_usize();

    let code = state.get_code(address);
    let calldata = memory.get_bytes(args_offset, args_size)?;
    let mut to = [0u8; 32];
    address.to_big_endian(&mut to);
    let mut value_bytes = [0u8; 32];
    value.to_big_endian(&mut value_bytes);
    let tx_data = TxData::new(vec![
        to.to_vec(),
        tx_to.to_vec(),
        tx_origin.to_vec(),
        vec![],
        value_bytes.to_vec(),
        calldata,
    ]);
    let block_data = BlockData::new(vec![]);

    let mut new_evm = Evm::new(
        Box::from(code),
        tx_data,
        block_data,
        state.clone(),
        storage.clone(),
        vec![],
        vec![],
        vec![],
        vec![],
        Memory::new(),
        limit,
        false,
    );

    let result = new_evm.execute();

    memory.save_bytes(ret_offset, &new_evm.return_data())?;
    *last_ret_data = new_evm.return_data();

    let res = match result {
        ExecutionResult::Success | ExecutionResult::Halt => {
            *state = new_evm.state();
            *storage = new_evm.storage();
            1.into()
        }
        ExecutionResult::Revert => 0.into(),
    };

    push(stack, res, limit)?;
    Ok(res)
}

pub fn delegatecall(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    state: &mut State,
    storage: &mut Storage,
    tx_to: &[u8],
    tx_from: &[u8],
    tx_origin: &[u8],
    value: &[u8],
    last_ret_data: &mut Vec<u8>,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let _gas = pop(stack)?;
    let address = pop(stack)?;
    let args_offset = pop(stack)?.as_usize();
    let args_size = pop(stack)?.as_usize();
    let ret_offset = pop(stack)?.as_usize();
    let _ret_size = pop(stack)?.as_usize();

    let code = state.get_code(address);
    let calldata = memory.get_bytes(args_offset, args_size)?;
    let mut to = [0u8; 32];
    address.to_big_endian(&mut to);
    let tx_data = TxData::new(vec![
        tx_to.to_vec(),
        tx_from.to_vec(),
        tx_origin.to_vec(),
        vec![],
        value.to_vec(),
        calldata,
    ]);
    let block_data = BlockData::new(vec![]);

    let mut new_evm = Evm::new(
        Box::from(code),
        tx_data,
        block_data,
        state.clone(),
        storage.clone(),
        vec![],
        vec![],
        vec![],
        vec![],
        Memory::new(),
        limit,
        false,
    );

    let result = new_evm.execute();

    memory.save_bytes(ret_offset, &new_evm.return_data())?;
    *last_ret_data = new_evm.return_data();

    let res = match result {
        ExecutionResult::Success | ExecutionResult::Halt => {
            *state = new_evm.state();
            *storage = new_evm.storage();
            1.into()
        }
        ExecutionResult::Revert => 0.into(),
    };

    push(stack, res, limit)?;
    Ok(res)
}

pub fn staticcall(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    state: &mut State,
    storage: &mut Storage,
    tx_to: &[u8],
    tx_origin: &[u8],
    tx_value: &[u8],
    last_ret_data: &mut Vec<u8>,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let _gas = pop(stack)?;
    let address = pop(stack)?;
    let args_offset = pop(stack)?.as_usize();
    let args_size = pop(stack)?.as_usize();
    let ret_offset = pop(stack)?.as_usize();
    let _ret_size = pop(stack)?.as_usize();

    let code = state.get_code(address);
    let calldata = memory.get_bytes(args_offset, args_size)?;
    let mut to = [0u8; 32];
    address.to_big_endian(&mut to);
    let tx_data = TxData::new(vec![
        to.to_vec(),
        tx_to.to_vec(),
        tx_origin.to_vec(),
        vec![],
        tx_value.to_vec(),
        calldata,
    ]);
    let block_data = BlockData::new(vec![]);

    let mut new_evm = Evm::new(
        Box::from(code),
        tx_data,
        block_data,
        state.clone(),
        storage.clone(),
        vec![],
        vec![],
        vec![],
        vec![],
        Memory::new(),
        limit,
        true,
    );

    let result = new_evm.execute();

    memory.save_bytes(ret_offset, &new_evm.return_data())?;
    *last_ret_data = new_evm.return_data();

    let res = match result {
        ExecutionResult::Success | ExecutionResult::Halt => {
            *state = new_evm.state();
            *storage = new_evm.storage();
            1.into()
        }
        ExecutionResult::Revert => 0.into(),
    };

    push(stack, res, limit)?;
    Ok(res)
}

pub fn create(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    state: &mut State,
    storage: &mut Storage,
    tx_to: &[u8],
    tx_origin: &[u8],
    last_ret_data: &mut Vec<u8>,
    limit: usize,
    read_only: bool,
) -> Result<U256, ExecutionError> {
    if read_only {
        return Err(ExecutionError::ReadOnly);
    }

    let value = pop(stack)?;
    let offset = pop(stack)?.as_usize();
    let size = pop(stack)?.as_usize();

    let code = memory.get_bytes(offset, size)?;
    let address = U256::from_big_endian(tx_to);
    let nonce = state.get_nonce(address);

    let contract_address = calculate_address(tx_to, nonce);
    let mut contract_address_bytes = [0u8; 32];
    contract_address.to_big_endian(&mut contract_address_bytes);

    let mut value_bytes = [0u8; 32];
    value.to_big_endian(&mut value_bytes);

    let tx_data = TxData::new(vec![
        contract_address_bytes.to_vec(),
        tx_to.to_vec(),
        tx_origin.to_vec(),
        vec![],
        value_bytes.to_vec(),
        vec![],
    ]);
    let block_data = BlockData::new(vec![]);

    let mut new_evm = Evm::new(
        Box::from(code),
        tx_data,
        block_data,
        state.clone(),
        storage.clone(),
        vec![],
        vec![],
        vec![],
        vec![],
        Memory::new(),
        limit,
        false,
    );

    let result = new_evm.execute();

    let res = match result {
        ExecutionResult::Success | ExecutionResult::Halt => {
            *state = new_evm.state();
            *storage = new_evm.storage();

            state.save_code(contract_address, new_evm.return_data(), value)?;
            *last_ret_data = new_evm.return_data();
            contract_address
        }
        ExecutionResult::Revert => 0.into(),
    };

    push(stack, res, limit)?;
    Ok(res)
}

pub fn calculate_address(sender_address: &[u8], nonce: usize) -> U256 {
    // no rlp encoding here... in a REAL EVM you should rlp encode [sender_address + nonce] before hashing
    let result = sha3_hash(&[sender_address, &nonce.to_be_bytes()].concat()).to_vec();
    let result = result.get(12..).unwrap_or(&[0_u8; 20]);
    let result = U256::from_big_endian(&result);

    result
}

pub fn selfdestruct(
    stack: &mut Vec<U256>,
    state: &mut State,
    tx_to: &[u8],
    read_only: bool,
) -> Result<(), ExecutionError> {
    if read_only {
        return Err(ExecutionError::ReadOnly);
    }
    let dest_address = pop(stack)?;
    let src_address = U256::from_big_endian(tx_to);

    let balance = state.get_balance(src_address);
    state.transfer_balance(balance, dest_address);
    state.delete_account(src_address);
    Ok(())
}
