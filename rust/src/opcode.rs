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
    Balance(49),
    Origin(50),
    Caller(51),
    Callvalue(52),
    Calldataload(53),
    Calldatasize(54),
    Calldatacopy(55),
    Codesize(56),
    Codecopy(57),
    Gasprice(58),
    Extcodesize(59),
    Extcodecopy(60),
    Extcodehash(63),
    Blockhash(64),
    Coinbase(65),
    Timestamp(66),
    Number(67),
    Difficulty(68),
    Gaslimit(69),
    Chainid(70),
    Selfbalance(71),
    Basfee(72),
    Mload(81),
    Mstore(82),
    Mstore8(83),
    Sload(84),
    Sstore(85),
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
    Push5(100),
    Push6(101),
    Push7(102),
    Push8(103),
    Push9(104),
    Push10(105),
    Push11(106),
    Push12(107),
    Push13(108),
    Push14(109),
    Push15(110),
    Push16(111),
    Push17(112),
    Push18(113),
    Push19(114),
    Push20(115),
    Push21(116),
    Push22(117),
    Push23(118),
    Push24(119),
    Push25(120),
    Push26(121),
    Push27(122),
    Push28(123),
    Push29(124),
    Push30(125),
    Push31(126),
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
    Log0(160),
    Log1(161),
    Log2(162),
    Log3(163),
    Log4(164),
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
            OpCode::Push5 => 5,
            OpCode::Push6 => 6,
            OpCode::Push7 => 7,
            OpCode::Push8 => 8,
            OpCode::Push9 => 9,
            OpCode::Push10 => 10,
            OpCode::Push11 => 11,
            OpCode::Push12 => 12,
            OpCode::Push13 => 13,
            OpCode::Push14 => 14,
            OpCode::Push15 => 15,
            OpCode::Push16 => 16,
            OpCode::Push17 => 17,
            OpCode::Push18 => 18,
            OpCode::Push19 => 19,
            OpCode::Push20 => 20,
            OpCode::Push21 => 21,
            OpCode::Push22 => 22,
            OpCode::Push23 => 23,
            OpCode::Push24 => 24,
            OpCode::Push25 => 25,
            OpCode::Push26 => 26,
            OpCode::Push27 => 27,
            OpCode::Push28 => 28,
            OpCode::Push29 => 29,
            OpCode::Push30 => 30,
            OpCode::Push31 => 31,
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

    /// Helper function to determine the number of topics of a `LOG` opcode
    pub fn topics(&self) -> usize {
        match self {
            OpCode::Log0 => 0,
            OpCode::Log1 => 1,
            OpCode::Log2 => 2,
            OpCode::Log3 => 3,
            OpCode::Log4 => 4,
            _ => 0, // return 0 for non-`LOG`
        }
    }

    pub fn is_push(&self) -> bool {
        OpCode::Push0 <= *self && *self <= OpCode::Push32
    }
}
