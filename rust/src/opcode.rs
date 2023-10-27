pub enum OpCode {
    Stop = 0,
    Push0 = 95,
    Push1 = 96,
    Push2 = 97,
    Push4 = 99,
    Push6 = 101,
}

impl TryFrom<u8> for OpCode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OpCode::Stop),
            95 => Ok(OpCode::Push0),
            96 => Ok(OpCode::Push1),
            97 => Ok(OpCode::Push2),
            99 => Ok(OpCode::Push4),
            101 => Ok(OpCode::Push6),
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
