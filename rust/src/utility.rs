use crate::errors::ExecutionError;
use primitive_types::U256;

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
