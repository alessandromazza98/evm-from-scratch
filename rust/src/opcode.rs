macro_rules! opcodes {
    ($($name:ident($number:expr),)*) => {
        #[derive(Debug, PartialEq, PartialOrd)]
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
    Pop(80),
    Add(1),
    Mul(2),
    Sub(3),
    Div(4),
    Sdiv(5),
    Mod(6),
    Smod(7),
    Addmod(8),
    Mulmod(9),
    Exp(10),
    Signextend(11),
    Lt(16),
    Gt(17),
    Slt(18),
    Sgt(19),
    Eq(20),
    Iszero(21),
    And(22),
    Or(23),
    Xor(24),
    Not(25),
    Byte(26),
    Shl(27),
    Shr(28),
    Sar(29),
    Sha3(32),
    Address(48),
    Origin(50),
    Caller(51),
    Gasprice(58),
    Coinbase(65),
    Timestamp(66),
    Number(67),
    Difficulty(68),
    Basfee(72),
    Mload(81),
    Mstore(82),
    Mstore8(83),
    Jump(86),
    Jumpi(87),
    Pc(88),
    Msize(89),
    Gas(90),
    Jumpdest(91),
    Push0(95),
    Push1(96),
    Push2(97),
    Push3(98),
    Push4(99),
    Push6(101),
    Push10(105),
    Push11(106),
    Push32(127),
    Dup1(128),
    Dup2(129),
    Dup3(130),
    Dup4(131),
    Dup5(132),
    Dup6(133),
    Dup7(134),
    Dup8(135),
    Dup9(136),
    Dup10(137),
    Dup11(138),
    Dup12(139),
    Dup13(140),
    Dup14(141),
    Dup15(142),
    Dup16(143),
    Swap1(144),
    Swap2(145),
    Swap3(146),
    Swap4(147),
    Swap5(148),
    Swap6(149),
    Swap7(150),
    Swap8(151),
    Swap9(152),
    Swap10(153),
    Swap11(154),
    Swap12(155),
    Swap13(156),
    Swap14(157),
    Swap15(158),
    Swap16(159),
}

impl OpCode {
    pub fn new(opcode: u8) -> Option<Self> {
        if let Ok(opcode) = opcode.try_into() {
            Some(opcode)
        } else {
            None
        }
    }

    /// Helper function to determine the push data size for each `Push` opcode
    pub fn push_data_size(&self) -> usize {
        match self {
            OpCode::Push1 => 1,
            OpCode::Push2 => 2,
            OpCode::Push3 => 3,
            OpCode::Push4 => 4,
            OpCode::Push6 => 6,
            OpCode::Push10 => 10,
            OpCode::Push11 => 11,
            OpCode::Push32 => 32,
            _ => 0, // return 0 for non-`PUSH` opcodes
        }
    }

    /// Helper function to determine the data to be duplicated for each `Dup` and `Swap`` opcode
    pub fn data_index(&self) -> usize {
        match self {
            OpCode::Dup1 | OpCode::Swap1 => 1,
            OpCode::Dup2 | OpCode::Swap2 => 2,
            OpCode::Dup3 | OpCode::Swap3 => 3,
            OpCode::Dup4 | OpCode::Swap4 => 4,
            OpCode::Dup5 | OpCode::Swap5 => 5,
            OpCode::Dup6 | OpCode::Swap6 => 6,
            OpCode::Dup7 | OpCode::Swap7 => 7,
            OpCode::Dup8 | OpCode::Swap8 => 8,
            OpCode::Dup9 | OpCode::Swap9 => 9,
            OpCode::Dup10 | OpCode::Swap10 => 10,
            OpCode::Dup11 | OpCode::Swap11 => 11,
            OpCode::Dup12 | OpCode::Swap12 => 12,
            OpCode::Dup13 | OpCode::Swap13 => 13,
            OpCode::Dup14 | OpCode::Swap14 => 14,
            OpCode::Dup15 | OpCode::Swap15 => 15,
            OpCode::Dup16 | OpCode::Swap16 => 16,
            _ => 0, // return 0 for non-`DUP` and non-`SWAP` opcodes
        }
    }

    pub fn is_push(&self) -> bool {
        OpCode::Push0 <= *self && *self <= OpCode::Push32
    }
}
