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
