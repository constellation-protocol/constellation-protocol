use crate::error::Error;
use crate::soroswap_router::router_pair_for;
use constellation_lib::traits::adapter::dex::IExchange;
use constellation_lib::traits::adapter::{self, dex};
use soroban_sdk::IntoVal;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, panic_with_error, vec, Address, BytesN, Env, String, Symbol, Val, Vec,
};

use crate::require::require_exchange_router;
static SWAP_TOKENS_FOR_EXACT_TOKENS: &'static str = "swap_tokens_for_exact_tokens";
static SWAP_EXACT_TOKENS_FOR_TOKEN: &'static str = "swap_exact_tokens_for_tokens";
static APPROVE: &'static str = "approve";
static TRANSFER: &'static str = "transfer";

pub fn get_swap_tokens_for_exact_tokens_call_data(
    e: &Env,
    token_in_id: Address,
    token_out_id: Address,
    amount_in: i128,
    amount_out: i128,
    to: Address,
    deadline: u64,
) -> (Symbol, Vec<Val>) {
    let path: Vec<Address> = vec![e, token_in_id.clone(), token_out_id.clone()];
    let mut args: Vec<Val> = vec![e];
    args.push_back(amount_out.into_val(e));
    args.push_back(amount_in.into_val(e));
    args.push_back(path.into_val(e));
    args.push_back(to.into_val(e));
    args.push_back(deadline.into_val(e));
    (Symbol::new(&e, SWAP_TOKENS_FOR_EXACT_TOKENS), args)
}

pub fn get_swap_exact_tokens_for_tokens_call_data(
    e: &Env,
    token_in_id: Address,
    token_out_id: Address,
    amount_in: i128,
    amount_out: i128,
    to: Address,
    deadline: u64,
) -> (Symbol, Vec<Val>) {
    let path: Vec<Address> = vec![e, token_in_id.clone(), token_out_id.clone()];
    let mut args: Vec<Val> = vec![e];
    args.push_back(amount_in.into_val(e));
    args.push_back(amount_out.into_val(e));
    args.push_back(path.into_val(e));
    args.push_back(to.into_val(e));
    args.push_back(deadline.into_val(e));
    (Symbol::new(&e, SWAP_EXACT_TOKENS_FOR_TOKEN), args)
}

pub fn create_sub_auth(
    e: &Env,
    amount_in: i128,
    token_in: Address,
    token_out: Address,
    from: Address,
    pair: Address,
) -> Vec<InvokerContractAuthEntry> {
    let mut args: Vec<Val> = vec![e];
    args.push_back(from.into_val(e));
    args.push_back(pair.into_val(e));
    args.push_back(amount_in.into_val(e));

    let mut sub_auth_vec = vec![&e];
    let pre_auth_entry = InvokerContractAuthEntry::Contract(SubContractInvocation {
        context: ContractContext {
            contract: token_in.clone(),
            fn_name: Symbol::new(&e, TRANSFER),
            args: args.clone(),
        },
        sub_invocations: vec![&e],
    });
    sub_auth_vec.push_back(pre_auth_entry);

    sub_auth_vec
}
