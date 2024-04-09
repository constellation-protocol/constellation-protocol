use soroban_sdk::{contracttype, Address, Symbol, Val, Vec};

#[derive(Clone)]
#[contracttype]
pub struct SubCalldata {
    pub contract_id: Address,
    pub function: Symbol,
    pub args: Vec<Val>,
    pub sub_auth: Vec<SubCalldata>,
}
