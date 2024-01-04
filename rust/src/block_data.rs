use primitive_types::U256;

/// Block data.
pub struct BlockData {
    pub basefee: U256,
    pub coinbase: U256,
    pub timestamp: U256,
    pub number: U256,
    pub difficulty: U256,
}

impl BlockData {
    pub fn new(block_data: Vec<Vec<u8>>) -> BlockData {
        if !block_data.is_empty() {
            Self {
                basefee: U256::from_big_endian(block_data[0].as_slice()),
                coinbase: U256::from_big_endian(block_data[1].as_slice()),
                timestamp: U256::from_big_endian(block_data[2].as_slice()),
                number: U256::from_big_endian(block_data[3].as_slice()),
                difficulty: U256::from_big_endian(block_data[4].as_slice()),
            }
        } else {
            Self {
                basefee: 0.into(),
                coinbase: 0.into(),
                timestamp: 0.into(),
                number: 0.into(),
                difficulty: 0.into()
            }
        }
    }
}
