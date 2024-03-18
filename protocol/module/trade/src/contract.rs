use soroban_sdk::{
    contract,   contractimpl, contracttype, 
     Address, Env, 
};

use crate::traits::adapter::{
    self,
    CallData
};

#[contract]
pub struct Trade{}

#[contractimpl]
impl Trade {
    
    pub fn initialize(e: Env) {

    }

    pub fn trade(e: Env, constellation_token_id: Address, exchange_id: Address, 
    token_in_id: Address, token_out_id: Address, amount_in: i128, amount_out: i128, to: Address, deadline: u64)  {
        // check adapter is registered (exchange_id)
        let exchange_adapter  = adapter::Client::new(&e, &exchange_id);
         let CallData { target ,method, data } = exchange_adapter.get_call_data();
         
    }
}