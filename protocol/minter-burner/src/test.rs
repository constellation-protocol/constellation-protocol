#![cfg(test)]

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    vec, BytesN, Env
};
use crate::contract::{constellation_token, MinterBurner, MinterBurnerClient };
 
// fn create_minter_burner<'a>(e: &Env) -> MinterBurnerClient<'a> {
//     let contract_id = &e.register_contract(None, crate::contract::MinterBurner {});
//     let ct: MinterBurnerClient<'_> = MinterBurnerClient::new(e, contract_id);
//     ct
// }

#[test]
fn test_mint() {
    let e = Env::default();
    // e.mock_all_auths();

    let c_token_id = e.register_contract_wasm(None, constellation_token::WASM);
   // let mn = create_minter_burner(&e);
} 