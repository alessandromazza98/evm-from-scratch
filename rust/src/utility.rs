use crate::{errors::ExecutionError, jumpdest::valid_jumpdest, memory::Memory};
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

    let mut value_bytes = [0u8; 32];
    value.to_big_endian(&mut value_bytes);

    // memory must have at least offset + 32 free bytes left.
    if memory.store.len() < offset + 32 {
        let resize_value = (offset + 31) / 32 + 1;
        if let Some(resize_value) = resize_value.checked_mul(32) {
            memory.store.resize(resize_value, 0);
        } else {
            return Err(ExecutionError::IntegerOverflow);
        }
    }

    for i in 0..32 {
        memory.store[offset + i] = value_bytes[i];
    }

    Ok(value)
}

pub fn mload(
    stack: &mut Vec<U256>,
    memory: &mut Memory,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let offset = pop(stack)?.as_usize();

    let mut value = vec![];

    // memory must have at least offset + 32 free bytes left.
    if memory.store.len() < offset + 32 {
        let resize_value = (offset + 31) / 32 + 1;
        if let Some(resize_value) = resize_value.checked_mul(32) {
            memory.store.resize(resize_value, 0);
        } else {
            return Err(ExecutionError::IntegerOverflow);
        }
    }

    for i in 0..32 {
        value.push(*memory.store.get(offset + i).unwrap_or(&0));
    }
    let value = U256::from_big_endian(value.as_slice());

    push(stack, value, limit)?;
    Ok(value.into())
}

pub fn mstore8(stack: &mut Vec<U256>, memory: &mut Memory) -> Result<U256, ExecutionError> {
    let offset = pop(stack)?.as_usize();
    let value = pop(stack)?;

    let mut value_bytes = [0u8; 32];
    value.to_big_endian(&mut value_bytes);

    if memory.store.len() < offset {
        let resize_value = offset / 32 + 1;
        if let Some(resize_value) = resize_value.checked_mul(32) {
            memory.store.resize(resize_value, 0);
        } else {
            return Err(ExecutionError::IntegerOverflow);
        }
    }

    memory.store[offset] = value_bytes[31];

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

    // create hash
    let mut hasher = Keccak256::new();
    hasher.update(value);
    let result = hasher.finalize();

    // convert into U256
    let result = U256::from_big_endian(&result);
    push(stack, result, limit)?;

    Ok(result)
}

pub fn address(
    stack: &mut Vec<U256>,
    to_address: U256,
    limit: usize,
) -> Result<U256, ExecutionError> {
    let address = to_address;

    push(stack, address, limit)?;
    Ok(address)
}
