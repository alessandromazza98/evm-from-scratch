# Day 7

## What I did

Today I dedicate myself a lot for this project and I added lots of opcodes:

- SWAP1-16
- DUP1-16
- BYTE
- PC
- JUMP
- JUMPDEST
- JUMPI
- GAS (simplified version of gas)
- MSTORE
- MSTORE8
- MLOAD
- MSIZE
- SHA3
- ADDRESS

### JUMP opcodes

It was very interesting to tackle these kinds of opcodes because of the JUMPDEST analysis which is
required to be sure that a certain destination is actually a correct JUMPDEST destination for the JUMP.

I use a `BitVec` to track what is code and what is data (push data) in the code so that it's easy to check
if a certain destination is a valid one:

1. If it's not a `JUMPDEST` opcode, it's not valid.
2. If it's not part of the code (and instead is a push data), it's not valid.

Otherwise it's valid.

### Memory

I finally added opcodes that work with the memory.

Memory, in the EVM, is a volatile memory which, essentialy, is an array of bytes.

I treat `Memory` as a struct containing a field called `store` that is a `Vec<u8>`.
It always starts empty and, everytime it is accessed (read or write), if the offset
where you should read / write in the memory is bigger than the length of the memory
itself, first the memory is resized and then it is read / write.

## To Do

There is some refactoring I could do to improve the code and delete some duplication:

- right now every time there is a `JUMP` opcode, the EVM analyze the code for jumpdest validation.
If there are more than one `JUMP` opcode in the same contract, the analysis is done again, although
it should just check the analysis done before! There should be a way to save the analysis and just
check it when it's needed.

- some refactoring on the memory opcodes. I would like to replace all the insert and resize operations
in the `utility.rs` file with some implementation functions in the `Memory` struct.
