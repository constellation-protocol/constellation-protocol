use soroban_sdk::{panic_with_error, Address, Env};

use crate::{error::Error, storage::factory::read_factory};

pub fn require_factory(e: &Env) -> Address {
    match read_factory(e) {
        Some(factory_id) => factory_id,
        None => panic_with_error!(&e, Error::RequiresFactory),
    }
}

pub fn require_pair(
    e: &Env,
    factory_id: Address,
    token_in: Address,
    token_out: Address,
) -> Address {
    match soroswap_library::pair_for(
        e.clone(),
        factory_id.clone(),
        token_in.clone(),
        token_out.clone(),
    ) {
        Ok(pair_id) => pair_id,
        Err(error) => panic_with_error!(&e, error),
    }
}
