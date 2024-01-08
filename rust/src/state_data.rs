use std::collections::HashMap;

use primitive_types::U256;

/// State data.
pub struct State {
    pub state: Vec<StateData>,
}

pub struct StateData {
    pub address: U256,
    pub data: AddressData,
}

pub struct AddressData {
    pub balance: Vec<u8>,
    pub code: Vec<u8>,
}

impl State {
    pub fn new(state_data: HashMap<Vec<u8>, (Vec<u8>, Vec<u8>)>) -> State {
        let mut state = vec![];
        for (address, data) in state_data {
            let state_data = StateData {
                address: U256::from_big_endian(&address),
                data: AddressData {
                    balance: data.0,
                    code: data.1,
                },
            };
            state.push(state_data);
        }
        State { state }
    }

    pub fn get_balance(&self, address: U256) -> Vec<u8> {
        self.state
            .iter()
            .find(|elem| elem.address == address)
            .map(|elem| elem.data.balance.clone())
            .unwrap_or_default()
    }
}
