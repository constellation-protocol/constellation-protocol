use super::DataKey;
use soroban_sdk::{Address, Env, Vec};
use super::{INSTANCE_LEDGER_LIFE, INSTANCE_LEDGER_TTL_THRESHOLD};

pub(crate) fn extend_ttl(e: &Env) {
    e.storage().instance().extend_ttl(INSTANCE_LEDGER_TTL_THRESHOLD, INSTANCE_LEDGER_LIFE);
 }
 
pub fn read_token_list(e: &Env) -> Vec<Address> {
    extend_ttl(e);
    let key = DataKey::TokenList;
    if let Some(token_list) = e.storage().instance().get(&key) {
        token_list
    } else {
        Vec::new(e)
    }
}

pub fn write_token_list(e: &Env, token_address: Address) {
    extend_ttl(e);
    let key = DataKey::TokenList;
    let mut tokens: Vec<Address> = read_token_list(e);
    tokens.push_back(token_address);
    e.storage().instance().set(&key, &tokens);
}
