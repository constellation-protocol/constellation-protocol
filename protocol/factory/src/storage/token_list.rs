use super::DataKey;
use soroban_sdk::{Address, Env, Vec};

pub fn read_token_list(e: &Env) -> Vec<Address> {
    let key = DataKey::TokenList;
    if let Some(token_list) = e.storage().instance().get(&key) {
        token_list
    } else {
        Vec::new(e)
    }
}

pub fn write_token_list(e: &Env, token_address: Address) {
    let key = DataKey::TokenList;
    let mut tokens: Vec<Address> = read_token_list(e);
    tokens.push_back(token_address);
    e.storage().instance().set(&key, &tokens);
    //  e.storage()
    //  .persistent()
    // .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
