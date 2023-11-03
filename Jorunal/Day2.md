# Day 1

## What I did

Today I managed to create the `opcodes!` macro that simplifies the creation of new opcodes since it automatically implements the `TryFrom<u8>` trait for `OpCode`. Now when you want to add a new opcode, you can simply add a new line inside the `opcodes!` declaration and then go in the `evm.rs` file and implement the correct function for that specific opcode.

After that I started implementing more opcodes, up to the `DIV` opcode. It was quite straight forward thanks to the `opcodes!` macro.

An interesting thing I had to change is the `evm.stack()` function that returns its stack. Since I am using a `Vec` to represent the EVM stack, when the stack has to be returned at the end of the code execution, it **MUST** be reversed so that it actually behaves as a normal stack.

```rust
/// Returns the stack at the end of execution. Note that the stack here is reversed.
    pub fn stack(&self) -> Vec<U256> {
        self.stack.iter().rev().cloned().collect()
    }
```

## To Do

I want to add some auxiliary functions that help a lot to reduce code duplication:

- one function to check if there are enough elements in the stack. Otherwise returns `ExecutionResult::Revert`.

- one function that takes an arbitrary number of elements from the stack, performs an operation and pushes the result on the stack.
