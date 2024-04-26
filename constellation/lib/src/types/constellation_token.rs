use soroban_sdk::{contracttype, Address};

#[derive(Clone, Debug)]
#[contracttype]
pub struct Component {
    pub address: Address,
    pub unit: i128,
}
