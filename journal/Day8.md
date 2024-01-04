# Day 8

## What I did

Today I added the following opcodes:

- ORIGIN
- CALLER
- GASPRICE
- COINBASE
- TIMESTAMP
- NUMBER
- DIFFICULTY

### Tx and Block data

Basically what I had to do today was to introduce to the EVM tx and block data.

I managed to do that creating a `TxData` and a `BlockData` struct.  

## To Do

I didn't work on what I said yesterday that I have to do, so I will re-write those two points and
then I'll add some other thing I'd like to improve:

- right now every time there is a `JUMP` opcode, the EVM analyze the code for jumpdest validation.
If there are more than one `JUMP` opcode in the same contract, the analysis is done again, although
it should just check the analysis done before! There should be a way to save the analysis and just
check it when it's needed.

- some refactoring on the memory opcodes. I would like to replace all the insert and resize operations
in the `utility.rs` file with some implementation functions in the `Memory` struct.

- right now I pass `TxData` and `BlockData` to the EVM. It could be interesting to consider a `Context` field
in the `Evm` struct, where `TxData` and `BlockData` are two fields of it.

- In `main.rs` I'd like to abstract a lot of the processing code, like all the `hex::decode...` lines of code.
The result should be to have a main function very simple and clean, abstracting away all the decoding and structuring parts.
