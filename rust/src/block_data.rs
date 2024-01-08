/// Block data.
pub struct BlockData {
    pub basefee: Vec<u8>,
    pub coinbase: Vec<u8>,
    pub timestamp: Vec<u8>,
    pub number: Vec<u8>,
    pub difficulty: Vec<u8>,
    pub gaslimit: Vec<u8>,
    pub chainid: Vec<u8>,
}

impl BlockData {
    pub fn new(block_data: Vec<Vec<u8>>) -> BlockData {
        if !block_data.is_empty() {
            Self {
                basefee: block_data[0].clone(),
                coinbase: block_data[1].clone(),
                timestamp: block_data[2].clone(),
                number: block_data[3].clone(),
                difficulty: block_data[4].clone(),
                gaslimit: block_data[5].clone(),
                chainid: block_data[6].clone(),
            }
        } else {
            Self {
                basefee: vec![],
                coinbase: vec![],
                timestamp: vec![],
                number: vec![],
                difficulty: vec![],
                gaslimit: vec![],
                chainid: vec![],
            }
        }
    }
}
