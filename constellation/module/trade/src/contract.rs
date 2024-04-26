use crate::error::Error;
use crate::token::{self, update_units};
use crate::{
    storage::registry::{has_registry, write_registry},
    validation::{require_adapter, require_administrator, require_manager, require_registry},
};
use constellation_lib::traits::adapter::dex;
use soroban_sdk::auth::InvokerContractAuthEntry;
use soroban_sdk::vec;
use soroban_sdk::{
    token::TokenClient,
    contract, contractimpl, contracttype, panic_with_error, Address, Env, Symbol, Val, Vec,
};
#[contract]
pub struct Trade {}

#[contractimpl]
impl Trade {
    pub fn initialize(e: Env, registry_id: Address) {
        if has_registry(&e) {
            panic_with_error!(&e, Error::AlreadyInitalized);
        }

        write_registry(&e, &registry_id);
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
        expiration_ledger: u32,
    ) -> Result<(), Error> {
        let manager = require_manager(&e, &constellation_token_id)?;
        manager.require_auth();

        let mut args: Vec<Val> = vec![&e];

        let registry_id = require_registry(&e)?;

        let adapter_id = require_adapter(&e, &registry_id, &exchange_id)?;

        let exchange_adapter = dex::Client::new(&e, &adapter_id);

        let approve_call_data = exchange_adapter.get_approve_call_data(
            &constellation_token_id,
            &exchange_id,
            &amount_in,
            &expiration_ledger,
        );

        Self::approve_exchange(
            &e,
            &constellation_token_id,
            &token_in_id,
            &approve_call_data,
        );

        let swap_call_data = exchange_adapter.get_swap_call_data(
            &token_in_id.clone(),
            &token_out_id.clone(),
            &amount_in,
            &amount_out,
            &constellation_token_id.clone(),
            &deadline,
        );

        let auth_entries = exchange_adapter.create_sub_auth(
            &amount_in,
            &token_in_id,
            &token_out_id,
            &constellation_token_id,
        );

        let balance_before_trade_token_in = TokenClient::new(&e, &token_in_id).balance(&constellation_token_id);
        let balance_before_trade_token_out = TokenClient::new(&e, &token_out_id).balance(&constellation_token_id);

        Self::execute_trade(
            &e,
            &constellation_token_id,
            &exchange_id,
            &swap_call_data,
            &auth_entries,
        );

        update_units(&e, balance_before_trade_token_in, balance_before_trade_token_out, &token_in_id, &token_out_id, &constellation_token_id);

        Ok(())
    }

    fn approve_exchange(
        e: &Env,
        constellation_token_id: &Address,
        token_id: &Address,
        call_data: &(Symbol /* function */, Vec<Val>),
    ) {
        token::invoke(&e, constellation_token_id, token_id, &call_data, &vec![e]);
    }
    fn execute_trade(
        e: &Env,
        constellation_token_id: &Address,
        exchange_id: &Address,
        call_data: &(Symbol /* function */, Vec<Val>),

        auth_entries: &Vec<InvokerContractAuthEntry>,
    ) {
        token::invoke(
            &e,
            constellation_token_id,
            exchange_id,
            &call_data,
            auth_entries,
        );
    }
}
