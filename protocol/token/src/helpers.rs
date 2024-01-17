use crate::component::read_components;
use crate::metadata::read_decimal;
use soroban_sdk::{token, Address, Env};

///  Lock - Transfers each component token to constellation token
///
///  # Arguments
///
/// - `e` The runtime environment.
/// - `from` Address of component tokens owner account
/// - `amount` amount of constellation token mint, which is multiplied by unit to obtain component value to transfer 
pub fn lock(e: &Env, from: &Address, amount: i128) {
    let components = read_components(e);
    for c in components.iter() {
        let quantity = amount * c.unit; // unit * amount
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

///  Release - Transfers each component token back to the `to` address
///
///  # Arguments
///
/// - `e` The runtime environment.
/// - `to` Address to send component tokens
/// - `amount` amount of constellation token  which is multiplied by  unit to obtain component value to transfer 
pub fn redeem(e: &Env, to: &Address, amount: i128) {
    let components = read_components(e);
    for c in components.iter() {
        let quantity = amount * c.unit; // c.unit * amount;
        let _token = token::Client::new(&e, &c.address);
        _token.transfer(&e.current_contract_address(), &to, &quantity);
    }
}
