use std::collections::HashMap;

use primitive_types::U256;

use crate::errors::ExecutionError;

/// State data.
#[derive(Debug, Clone)]
pub struct State {
    pub state: Vec<StateData>,
}

#[derive(Debug, Clone)]
pub struct StateData {
    pub address: U256,
    pub data: AddressData,
}

impl StateData {
    pub fn new(address: U256, balance: U256, code: Vec<u8>) -> StateData {
        let address_data = AddressData {
            nonce: 0,
            balance,
            code,
        };
        StateData {
            address: address,
            data: address_data,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddressData {
    pub nonce: usize,
    pub balance: U256,
    pub code: Vec<u8>,
}

impl State {
    pub fn new(state_data: HashMap<Vec<u8>, (usize, Vec<u8>, Vec<u8>)>) -> State {
        let mut state = vec![];
        for (address, data) in state_data {
            let state_data = StateData {
                address: U256::from_big_endian(&address),
                data: AddressData {
                    nonce: data.0,
                    balance: U256::from(data.1.as_slice()),
                    code: data.2,
                },
            };
            state.push(state_data);
        }
        State { state }
    }

    pub fn get_balance(&self, address: U256) -> U256 {
        self.state
            .iter()
            .find(|elem| elem.address == address)
            .map(|elem| elem.data.balance.clone())
            .unwrap_or_default()
    }

    pub fn get_code(&self, address: U256) -> Vec<u8> {
        self.state
            .iter()
            .find(|elem| elem.address == address)
            .map(|elem| elem.data.code.clone())
            .unwrap_or_default()
    }

    pub fn get_nonce(&self, address: U256) -> usize {
        self.state
            .iter()
            .find(|elem| elem.address == address)
            .map(|elem| elem.data.nonce.clone())
            .unwrap_or_default()
    }

    pub fn save_code(
        &mut self,
        address: U256,
        code: Vec<u8>,
        value_transferred: U256,
    ) -> Result<(), ExecutionError> {
        match self.state.iter().find(|elem| elem.address == address) {
            Some(_state) => Err(ExecutionError::ContractAddressCollision),
            None => {
                let address_data = AddressData {
                    nonce: 0,
                    code,
                    balance: value_transferred,
                };
                let state_data = StateData {
                    address,
                    data: address_data,
                };
                self.state.push(state_data);
                Ok(())
            }
        }
    }

    pub fn delete_account(&mut self, address: U256) {
        self.state.retain(|account| account.address != address);
    }

    pub fn transfer_balance(&mut self, balance: U256, dest: U256) {
        if let Some(account) = self.state.iter_mut().find(|elem| elem.address == dest) {
            account.data.balance += balance;
        } else {
            let new_account = StateData::new(dest, balance, vec![]);
            self.state.push(new_account);
        }
    }
}
