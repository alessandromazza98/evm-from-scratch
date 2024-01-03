use primitive_types::U256;

/// Tx data.
pub struct TxData {
    pub to: U256,
}

impl TxData {
    pub fn new(to: impl AsRef<[u8]>) -> TxData {
        Self {
            to: to.as_ref().into(),
        }
    }
}
