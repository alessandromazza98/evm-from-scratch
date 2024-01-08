# Day 9

## What I did

Today I added the following opcodes:

- CALLVALUE
- CALLDATALOAD
- CALLDATASIZE
- CALLDATACOPY
- BLOCKHASH
- GASLIMIT
- CHAINID
- BALANCE

### Push from big endian

I removed a lot of duplicated lines of code that were just handling a push operation from big endian bytes.
I created the `push_from_big_endian` function for that specific purpose.

### Refactor memory

First of all I refactored a bit the `Memory`, adding `save_word`, `save_byte`, and `get_word` that automatically call
the `resize` operations when needed.

This is very helpful because whenever I need to use the memory, I do not need to worry about memory expansion.
It's automatically handled by those functions.

### State data

I added `State` to handle the state.

Basically the state is a vector of `StateData`, which is made by:

- an address
- an `AddressData`: balance + code

## To Do

The following points are what I would like to do before completing the challenge:

- right now every time there is a `JUMP` opcode, the EVM analyze the code for jumpdest validation.
If there are more than one `JUMP` opcode in the same contract, the analysis is done again, although
it should just check the analysis done before! There should be a way to save the analysis and just
check it when it's needed.

- right now I pass `TxData` and `BlockData` to the EVM. It could be interesting to consider a `Context` field
in the `Evm` struct, where `TxData` and `BlockData` are two fields of it. NOTE: The `State` could be part of it too.

- In `main.rs` I'd like to abstract a lot of the processing code, like all the `hex::decode...` lines of code.
The result should be to have a main function very simple and clean, abstracting away all the decoding and structuring parts.
