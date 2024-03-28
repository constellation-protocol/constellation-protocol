use crate::error::Error;
use crate::token;
use crate::traits::adapter::{self, CallData};
use crate::{
    storage::registry::{has_registry, write_registry},
    validation::{require_adapter, require_administrator, require_manager, require_registry},
};
use soroban_sdk::{contract, contractimpl, contracttype, panic_with_error, Address, Env};
#[contract]
pub struct Trade {}

#[contractimpl]
impl Trade {
    pub fn initialize(e: Env, id: Address) {
        if has_registry(&e) {
            panic_with_error!(&e, Error::AlreadyInitalized);
        }

        write_registry(&e, &id);
    }
    pub fn trade(
        e: Env,
        constellation_token_id: Address,
        exchange_id: Address,
        token_in_id: Address,
        token_out_id: Address,
        amount_in: i128,
        amount_out: i128,
        deadline: u64,
    ) -> Result<(), Error> {
        let manager = require_manager(&e, &constellation_token_id)?;
        manager.require_auth();

        let registry_id = require_registry(&e)?;

        let adapter_id = require_adapter(&e, &registry_id, &exchange_id)?;

        let exchange_adapter = adapter::Client::new(&e, &adapter_id);
        let call_data = exchange_adapter.get_call_data(
            &token_in_id,
            &token_out_id,
            &amount_in,
            &amount_out,
            &constellation_token_id,
            &deadline,
        );

        Self::_trade(&e, &constellation_token_id, &exchange_id, &call_data);
        Ok(())
    }
    fn _trade(
        e: &Env,
        constellation_token_id: &Address,
        exchange_id: &Address,
        CallData { function, data }: &CallData,
    ) {
        token::invoke(&e, constellation_token_id, exchange_id, &function, &data);
    }
}
