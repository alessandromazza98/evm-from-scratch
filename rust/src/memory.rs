/// The memory for the EVM.
pub struct Memory {
    pub store: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { store: Vec::new() }
    }
}
