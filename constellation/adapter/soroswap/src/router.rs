use soroban_sdk::{vec, Address, Env, Vec};

use crate::error::Error;

mod router {
    soroban_sdk::contractimport!(file = "../../../libs/soroswap_router.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}

pub(crate) fn router_pair_for(
    e: &Env,
    router_id: &Address,
    token_a: &Address,
    token_b: &Address,
) -> Address {
    let ctoken = router::Client::new(&e, router_id);
    ctoken.router_pair_for(token_a, token_b)
}
