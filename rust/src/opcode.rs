macro_rules! opcodes {
    ($($name:ident($number:expr),)*) => {
        #[derive(Debug)]
        pub enum OpCode {
            $($name = $number,)*
        }

        impl TryFrom<u8> for OpCode {
            type Error = String;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(OpCode::$name),)*
                    _ => Err("Unknown opcode".to_string()),
                }
            }
        }
    };
}

opcodes! {
    Stop(0),
    Push0(95),
    Push1(96),
    Push2(97),
    Push4(99),
    Push6(101),
    Push10(105),
    Push11(106),
    Push32(127),
    Pop(80),
    Add(1),
    Mul(2),
    Sub(3),
    Div(4),
    Mod(6),
}

impl OpCode {
    pub fn new(opcode: u8) -> Option<Self> {
        if let Ok(opcode) = opcode.try_into() {
            Some(opcode)
        } else {
            None
        }
    }

    /// Helper function to determine the push data size for each `Push`` opcode
    pub fn push_data_size(&self) -> usize {
        match self {
            OpCode::Push1 => 1,
            OpCode::Push2 => 2,
            OpCode::Push4 => 4,
            OpCode::Push6 => 6,
            OpCode::Push10 => 10,
            OpCode::Push11 => 11,
            OpCode::Push32 => 32,
            _ => 0, // return 0 for non-`PUSH` opcodes
        }
    }
}
