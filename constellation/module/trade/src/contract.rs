use soroban_sdk::{contract, contractimpl, panic_with_error,contracttype, Address, Env};
use crate::storage::{
    adapter::{read_or_panic_unregistered_adapter, read_adapter, remove_adapter, write_adapter}, admin::{has_administrator, read_administrator, write_administrator}};
use crate::traits::adapter::{self, CallData};
use crate::token;
use crate::error::Error;

#[contract]
pub struct Trade {}

#[contractimpl]
impl Trade {
    pub fn initialize(e: Env, id: Address) {
        if has_administrator(&e) {
            panic_with_error!(&e, Error::AlreadyInitalized);
        }

         write_administrator(&e, &id);
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
    ) {

        let adapter_id = read_or_panic_unregistered_adapter(&e, &exchange_id);

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
    }

    pub fn add_adapter(e: Env,  exchange_id: Address, adapter_id: Address) -> Result<(), Error> {
        match read_administrator(&e) {
            Some(admin) => admin.require_auth(),
            None => return Err(Error::RequiresAdmin),
        }

        write_adapter(&e, exchange_id, adapter_id);
        Ok(())
    }

    pub fn remove_adapter(e: Env, exchange_id: Address) -> Result<(), Error> {
        match read_administrator(&e) {
            Some(admin) => admin.require_auth(),
            None => return Err(Error::RequiresAdmin),
        }

        remove_adapter(&e, exchange_id);
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
