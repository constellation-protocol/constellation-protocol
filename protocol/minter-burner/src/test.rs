#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, Address, BytesN, Env, Val,
};

use crate::{
    contract::{constellation_token, MinterBurner, MinterBurnerClient},
    error::Error,
};
use soroban_sdk::IntoVal;

pub mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}
fn create_constellation_token<'a>(e: &Env) -> (constellation_token::Client<'a>, Address) {
    let contract_id = &e.register_contract_wasm(None, constellation_token::WASM);
    let ct: constellation_token::Client<'_> = constellation_token::Client::new(e, contract_id);
    (ct, contract_id.clone())
}
fn create_minter_burner<'a>(e: &Env) -> MinterBurnerClient<'a> {
    let contract_id = &e.register_contract(None, crate::contract::MinterBurner {});
    let ct: MinterBurnerClient<'_> = MinterBurnerClient::new(e, contract_id);
    ct
}

#[test]
fn test_mint() {
    let e = Env::default();
    e.mock_all_auths();

    let mut admin1 = Address::generate(&e);

    let token1 = create_token_contract(&e, &admin1);
    let token2 = create_token_contract(&e, &admin1);

    let user1 = Address::generate(&e);

    token1.mint(&user1, &5000);
    token2.mint(&user1, &5000);
    let components = vec![&e, token1.address.clone(), token2.address.clone()];

    let minter_burner = create_minter_burner(&e);

    let amounts = vec![&e, 100, 100];
    let decimal: u32 = 6;
    let name = "c_token".into_val(&e);
    let symbol = "token_symbol".into_val(&e);
    let manager = Address::generate(&e);
    let (ct, ct_id) = create_constellation_token(&e);

    ct.initialize(
        &decimal,
        &components,
        &amounts,
        &name,
        &symbol,
        &minter_burner.address,
        &manager,
    );

    token1.approve(&user1, &ct.address, &5000i128, &1000);
    token2.approve(&user1, &ct.address, &5000i128, &1000);

    let amount = 1;
    let result = minter_burner.try_mint(&user1, &ct_id, &amount);

    assert_eq!(result, Ok(Ok(()))); 
}
