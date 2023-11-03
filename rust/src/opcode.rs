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
