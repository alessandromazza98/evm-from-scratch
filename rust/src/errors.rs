use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("halt the execution")]
    Halt,
    #[error("there are not enough items in the stack")]
    StackUnderflow,
    #[error("there are not enough items in the code to be pushed")]
    InsufficientCodeItems,
    #[error("the EVM stack can hold up to 1024 elements")]
    StackOverflow,
    #[error("not a valid jump destination")]
    NotValidJumpDestination,
    #[error("invalid opcode")]
    InvalidOpcode,
    #[error("integer overflow")]
    IntegerOverflow,
    #[error("revert opcode")]
    Revert,
    #[error("execution is read only")]
    ReadOnly,
}
