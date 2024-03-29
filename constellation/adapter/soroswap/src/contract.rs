use constellation_lib::traits;
use soroban_sdk::IntoVal;
use soroban_sdk::{contract, contractimpl, vec, Address, BytesN, Env, String, Symbol, Val, Vec};

#[contract]
pub struct SoroswapAdapter;

impl SoroswapAdapter {
    fn get_swap_data(
        e: &Env,
        token_in_id: Address,
        token_out_id: Address,
        amount_in: i128,
        amount_out: i128,
        to: Address,
        deadline: u64,
    ) -> (Symbol, Vec<Val>) {
        let mut args: Vec<Val> = vec![e];
        args.push_back(token_in_id.into_val(e));
        args.push_back(token_out_id.into_val(e));
        args.push_back(amount_in.into_val(e));
        args.push_back(amount_out.into_val(e));
        args.push_back(to.into_val(e));
        args.push_back(deadline.into_val(e));
        (Symbol::new(&e, "swap_tokens_for_exact_tokens"), args)
    }

    fn get_approve_data(e: &Env, approve_amount: i128, spender: Address) -> (Symbol, Vec<Val>) {
        let mut args: Vec<Val> = vec![e];
        args.push_back(spender.into_val(e));
        args.push_back(approve_amount.into_val(e));
        (Symbol::new(&e, "approve"), args)
    }
}

impl traits::adapter::dex::Interface for SoroswapAdapter {
    fn get_call_data(
        e: &Env,
        token_in_id: Address,
        token_out_id: Address,
        amount_in: i128,
        amount_out: i128,
        to: Address,
        deadline: u64,
        spender: Address,
    ) -> Vec<(Symbol, Vec<Val>)> {
        let approve_data = SoroswapAdapter::get_approve_data(e, amount_in, spender);
        let swap_data = SoroswapAdapter::get_swap_data(
            e,
            token_in_id,
            token_out_id,
            amount_in,
            amount_out,
            to,
            deadline,
        );
        let call_data: Vec<(Symbol, Vec<Val>)> = vec![&e, approve_data, swap_data];
        call_data
    }
}
