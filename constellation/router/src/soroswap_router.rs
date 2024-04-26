use soroban_sdk::{vec, Address, Env, Vec};

use crate::error::Error;

mod router {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_router.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}

pub fn router_get_amounts_in(
    e: &Env,
    amount_out: i128,
    router_id: &Address,
    path: &Vec<Address>,
) -> Vec<i128> {
    let router = router::Client::new(e, router_id);
    router.router_get_amounts_in(&amount_out, path)
}

pub fn swap_exact_tokens_for_tokens(
    e: &Env,
    router_id: &Address,
    amount_in: i128,
    amount_out_min: i128,
    token_in: &Address,
    token_out: &Address,
    to: &Address,
    deadline: u64,
) -> Result<i128, Error> {
    let path: Vec<Address> = vec![e, token_in.clone(), token_out.clone()];
    let router = router::Client::new(e, router_id);
    let swap =
        router.swap_exact_tokens_for_tokens(&amount_in, &amount_out_min, &path, &to, &deadline);
    match swap.get(1) {
        Some(value) => Ok(value),
        None => return Err(Error::SwapError),
    }
}

pub fn swap_tokens_for_exact_tokens(
    e: &Env,
    router_id: &Address,
    amount_out: i128,
    amount_in_max: i128,
    token_in: &Address,
    token_out: &Address,
    to: &Address,
    deadline: u64,
) -> Result<i128, Error> {
    let path: Vec<Address> = vec![e, token_in.clone(), token_out.clone()];
    let router = router::Client::new(e, router_id);
    let swap =
        router.swap_tokens_for_exact_tokens(&amount_out, &amount_in_max, &path, to, &deadline);
    match swap.get(0) {
        Some(value) => Ok(value),
        None => return Err(Error::SwapError),
    }
}
