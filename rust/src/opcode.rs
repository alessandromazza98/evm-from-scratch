pub enum OpCode {
    STOP = 0,
    PUSH0 = 95,
    PUSH1 = 96,
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::STOP),
            95 => Ok(OpCode::PUSH0),
            96 => Ok(OpCode::PUSH1),
            _ => Err("Unknown opcode".to_string()),
        }
    }
}

impl OpCode {
    pub fn new(opcode: u8) -> Option<Self> {
        if let Ok(opcode) = opcode.try_into() {
            Some(opcode)
        } else {
            None
        }
    }
}
