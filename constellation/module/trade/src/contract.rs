use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

use crate::traits::adapter::{self, CallData};

#[contract]
pub struct Trade {}

#[contractimpl]
impl Trade {
    pub fn initialize(e: Env) {}

    pub fn trade(
        e: Env,
        constellation_token_id: Address,
        target_exchange_id: Address,
        token_in_id: Address,
        token_out_id: Address,
        amount_in: i128,
        amount_out: i128,
        deadline: u64,
    ) {
        // check adapter is registered (exchange_id)
        let exchange_adapter = adapter::Client::new(&e, &target_exchange_id);
        let call_data = exchange_adapter.get_call_data(
            &token_in_id,
            &token_out_id,
            &amount_in,
            &amount_out,
            &constellation_token_id,
            &deadline,
        );

        Self::_trade(&e, &constellation_token_id, &target_exchange_id, &call_data);
    }
    //     fn invoke(e: Env, module_id: Address, target_contract_id: Address, function: Symbol, args: Vec<Val>)
    fn _trade(
        e: &Env,
        constellation_token_id: &Address,
        target_exchange_id: &Address,
        CallData { function, data }: &CallData,
    ) {
    }
}
