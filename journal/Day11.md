# Day 11

## What I did

Today I added the following opcodes:

- CALL
- RETURN
- DELEGATECALL
- STATICCALL
- REVERT
- RETURNDATASIZE
- RETURNDATACOPY

## To Do

Nothing different from yesterday here... I just copy paste here so that it's easier for me to know what
I have to do just by looking at the last journal.

The following points are what I would like to do before completing the challenge:

- right now every time there is a `JUMP` opcode, the EVM analyze the code for jumpdest validation.
If there are more than one `JUMP` opcode in the same contract, the analysis is done again, although
it should just check the analysis done before! There should be a way to save the analysis and just
check it when it's needed.

- right now I pass `TxData` and `BlockData` to the EVM. It could be interesting to consider a `Context` field
in the `Evm` struct, where `TxData` and `BlockData` are two fields of it. NOTE: The `State` could be part of it too.

- In `main.rs` I'd like to abstract a lot of the processing code, like all the `hex::decode...` lines of code.
The result should be to have a main function very simple and clean, abstracting away all the decoding and structuring parts.
