/// Tx data.
pub struct TxData {
    pub to: Vec<u8>,
    pub from: Vec<u8>,
    pub origin: Vec<u8>,
    pub gasprice: Vec<u8>,
    pub value: Vec<u8>,
    pub data: Vec<u8>,
}

impl TxData {
    pub fn new(tx_data: Vec<Vec<u8>>) -> TxData {
        if !tx_data.is_empty() {
            Self {
                to: tx_data[0].clone(),
                from: tx_data[1].clone(),
                origin: tx_data[2].clone(),
                gasprice: tx_data[3].clone(),
                value: tx_data[4].clone(),
                data: tx_data[5].clone(),
            }
        } else {
            Self {
                to: vec![],
                from: vec![],
                origin: vec![],
                gasprice: vec![],
                value: vec![],
                data: vec![],
            }
        }
    }
}
