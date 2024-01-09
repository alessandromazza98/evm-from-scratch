use primitive_types::U256;

#[derive(Debug, Clone, PartialEq)]
pub struct Log {
    pub address: U256,
    pub data: Vec<u8>,
    pub topics: Vec<U256>,
}

impl Log {
    pub fn new(address: U256, data: Vec<u8>, topics: Vec<U256>) -> Log {
        Log {
            address,
            data,
            topics,
        }
    }
}
