# Day 1

## What I did

I started creating my EVM and I'm quite satisfied of it. Right now it passes every PUSH test. I also created a `push_x` function in order to not write duplicate code.

The structure of my EVM is the following:

- `Evm`: it's the core struct and represents the EVM. It has two fields:

  - `code`: the EVM bytecode.
  - `stack`: the EVM stack.

To compute an EVM execution:

First you have to create the EVM:

```rust
let mut evm = Evm::new(Box::from(code), Vec::new());
```

Then you can execute it:

```rust
evm.execute()
```

This function takes the bytecode and for every byte does the following:

- check if the byte corresponds to an actual known opcode, otherwise reverts.

- call `evm.transact(opcode)` in order to perform the actual computation of the opcode.

At the end it returns an `ExecutionResult` that can be:

- `Success`
- `Halt`
- `Revert`

## To Do

First I want to write a macro that helps me in the creation of new opcodes. Right now it’s very tedious and repetitive when I have to add a new one.
