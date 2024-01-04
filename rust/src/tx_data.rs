use primitive_types::U256;

/// Tx data.
pub struct TxData {
    pub to: U256,
    pub from: U256,
    pub origin: U256,
    pub gasprice: U256,
}

impl TxData {
    pub fn new(tx_data: Vec<Vec<u8>>) -> TxData {
        if !tx_data.is_empty() {
            Self {
                to: U256::from_big_endian(tx_data[0].as_slice()),
                from: U256::from_big_endian(tx_data[1].as_slice()),
                origin: U256::from_big_endian(tx_data[2].as_slice()),
                gasprice: U256::from_big_endian(tx_data[3].as_slice()),
            }
        } else {
            Self {
                to: 0.into(),
                from: 0.into(),
                origin: 0.into(),
                gasprice: 0.into(),
            }
        }
    }
}
