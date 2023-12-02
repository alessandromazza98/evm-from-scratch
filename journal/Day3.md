# Day 3

## What I did

I'm very happy, I just added a new opcode `MOD` but I did a big refactor of the code.

The `evm.transact()` function now returns a `Result<(), ExecutionError` so that every utility function inside `utility.rs` can be used as a real helper function and does not have to immediately return an `ExecutionResult`.

I can use every utility function inside of other utility functions. For example:

- `push` function is now used every time a new item has to be pushed inside the stack. It checks if the stack is not full, otherwise returns a `StackOverflow` error.

- `pop` function is now used every time you have to pop an item from the stack. It checks if the stack has enough itmes to be popped, otherwise returns a `StackUnderflow` error.

This way every time there is an opcode that takes some elements from the stack, performs a computation and then pushes the result on the stack, I can use the previously mentioned functions.

I also reduced by a lot the duplication of code for `PUSH` opcodes:

- I created an helper function `push_data_size` that returns for every `PUSH` opcode how many bytes of data you have to push.

- Then I created a function that takes the whole code and returns the item to be pushed on the stack.

- And finally there is the previously mentioned `push` function to effectively add the item on the stack.

## To Do

Tomorrow I just want to add lots of new opcodes. I don't have any other improvement to be done for now.
