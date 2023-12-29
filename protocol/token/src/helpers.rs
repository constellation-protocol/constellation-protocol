use crate::component::read_components;
use soroban_sdk::{token, Address, Env};

pub fn lock(e: &Env, from: &Address, amount: i128) {
    let components = read_components(e);
    for c in components.iter() {
        let quantity = c.unit * amount; // unit * amount
        let _token = token::Client::new(e, &c.address);
        let b = _token.balance(from);
        _token.transfer_from(
            &e.current_contract_address(),
            from,
            &e.current_contract_address(),
            &quantity,
        );
    }
}

pub fn redeem(e: &Env, to: &Address, amount: i128) {
    let components = read_components(e);
    for c in components.iter() {
        let quantity = c.unit * amount;
        let _token = token::Client::new(&e, &c.address);
        _token.transfer(&e.current_contract_address(), &to, &quantity);
    }
}
