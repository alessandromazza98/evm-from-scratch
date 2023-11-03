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

pub fn pop_x(x: usize, pc: &mut usize, stack: &mut Vec<U256>) -> ExecutionResult {
    if stack.len() < x {
        ExecutionResult::Revert
    } else {
        for _ in 0..x {
            stack.pop();
        }
        *pc += 1;
        ExecutionResult::Success
    }
}

pub fn add(pc: &mut usize, stack: &mut Vec<U256>) -> ExecutionResult {
    if stack.len() < 2 {
        ExecutionResult::Revert
    } else {
        let a = stack
            .pop()
            .expect("the stack should have at least two items!");
        let b = stack
            .pop()
            .expect("the stack should have at least two items!");
        // we don't care if it overflows. EVM simply wraps around.
        let (sum, _) = a.overflowing_add(b);
        stack.push(sum);
        *pc += 1;
        ExecutionResult::Success
    }
}

pub fn mul(pc: &mut usize, stack: &mut Vec<U256>) -> ExecutionResult {
    if stack.len() < 2 {
        ExecutionResult::Revert
    } else {
        let a = stack
            .pop()
            .expect("the stack should have at least two items!");
        let b = stack
            .pop()
            .expect("the stack should have at least two items!");
        // we don't care if it overflows. EVM simply wraps around.
        let (mul, _) = a.overflowing_mul(b);
        stack.push(mul);
        *pc += 1;
        ExecutionResult::Success
    }
}

pub fn sub(pc: &mut usize, stack: &mut Vec<U256>) -> ExecutionResult {
    if stack.len() < 2 {
        ExecutionResult::Revert
    } else {
        let a = stack
            .pop()
            .expect("the stack should have at least two items!");
        let b = stack
            .pop()
            .expect("the stack should have at least two items!");
        // we don't care if it overflows. EVM simply wraps around.
        let (sub, _) = a.overflowing_sub(b);
        stack.push(sub);
        *pc += 1;
        ExecutionResult::Success
    }
}

pub fn div(pc: &mut usize, stack: &mut Vec<U256>) -> ExecutionResult {
    if stack.len() < 2 {
        ExecutionResult::Revert
    } else {
        let a = stack
            .pop()
            .expect("the stack should have at least two items!");
        let b = stack
            .pop()
            .expect("the stack should have at least two items!");
        if b == 0.into() {
            // if denominator is 0, returns 0.
            stack.push(0.into());
        } else {
            // we don't care if it overflows. EVM simply wraps around.
            let (sub, _) = a.div_mod(b);
            stack.push(sub);
        }
        *pc += 1;
        ExecutionResult::Success
    }
}
