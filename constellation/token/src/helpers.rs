use crate::storage::component::{
    read_component, read_components_list, remove_component, write_component,
};
use crate::storage::metadata::read_decimal;
use crate::storage::total_supply::{read_total_supply, write_total_supply};
use crate::storage::types::Component;
use soroban_sdk::token::TokenClient;
use soroban_sdk::{token, Address, Env};

///  Lock - Transfers each component token to constellation token
///
///  # Arguments
///
/// - `e` The runtime environment.
/// - `from` Address of component tokens owner account
/// - `amount` amount of constellation token mint, which is multiplied by unit to obtain component value to transfer
pub fn lock(e: &Env, from: &Address, amount: i128) {
    let components = read_components_list(e);
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
    let components = read_components_list(e);
    for c in components.iter() {
        let quantity = amount * c.unit; // c.unit * amount;
        let _token = token::Client::new(&e, &c.address);
        _token.transfer(&e.current_contract_address(), &to, &quantity);
    }
}

pub fn increase_supply(e: &Env, amount: i128) {
    let mut new_total_supply = read_total_supply(&e);
    write_total_supply(&e, new_total_supply + amount);
}

pub fn decrease_supply(e: &Env, amount: i128) {
    let mut total_supply = read_total_supply(&e);

    total_supply = if total_supply > amount {
        total_supply - amount
    } else {
        0
    };
    write_total_supply(&e, total_supply);
}

#[inline(always)]
pub fn calculate_airdropped_amount(
    component_previous_balance: i128,
    component_unit: i128,
    constellation_token_supply: i128,
) -> i128 {
    component_previous_balance - (component_unit * constellation_token_supply)
}

#[inline(always)]
pub fn calculate_position(
    component_total_balance: i128,
    constellation_token_supply: i128,
    airdropped_amount: i128,
) -> i128 {
    (component_total_balance - airdropped_amount) / constellation_token_supply
}

pub fn update_position(
    e: &Env,
    (component_address, component_previous_balance): (Address, i128),
) -> i128 {
    let component_current_balance =
        TokenClient::new(&e, &component_address).balance(&e.current_contract_address());
    let constellation_token_supply = read_total_supply(&e);

    let unit = match read_component(&e, component_address.clone()) {
        Some(mut component) => {
            let airdropped_amount = calculate_airdropped_amount(
                component_previous_balance,
                component.unit,
                constellation_token_supply,
            );
            let unit = calculate_position(
                component_current_balance,
                constellation_token_supply,
                airdropped_amount,
            );

            if unit == 0 {
                remove_component(&e, component_address.clone());
            } else {
                write_component(
                    &e,
                    component_address.clone(),
                    Component { unit, ..component },
                );
            }
            unit
        }
        None => {
            let airdropped_amount = calculate_airdropped_amount(
                component_previous_balance,
                0,
                constellation_token_supply,
            );
            let unit = calculate_position(
                component_current_balance,
                constellation_token_supply,
                airdropped_amount,
            );

            if unit > 0 {
                write_component(
                    &e,
                    component_address.clone(),
                    Component {
                        address: component_address,
                        unit,
                    },
                );
            }
            unit
        }
    };

    unit
}
