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

    if b == 0.into() {
        // if denominator is 0, returns 0.
        let div = 0.into();
        push(stack, div, limit)?;
        Ok(0.into())
    } else {
        // we don't care if it overflows. EVM simply wraps around.
        let (div, _) = a.div_mod(b);
        push(stack, div, limit)?;
        Ok(div)
    }
}

pub fn mod_fn(stack: &mut Vec<U256>, limit: usize) -> Result<U256, ExecutionError> {
    let a = pop(stack)?;
    let b = pop(stack)?;

    if b == 0.into() {
        // if the denominator is 0, the result will be 0.
        let result = 0.into();
        push(stack, result, limit)?;
        Ok(0.into())
    } else {
        let result = a % b;
        push(stack, result, limit)?;
        Ok(result)
    }
}
