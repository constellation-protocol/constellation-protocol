use soroban_sdk::{panic_with_error, Address, Env};

use crate::{
    error::Error,
    storage::{read_exchange_router, read_xlm},
};

pub fn require_exchange_router(e: &Env) -> Address {
    match read_exchange_router(e) {
        Some(router_id) => router_id,
        None => panic_with_error!(&e, Error::RequiresExchangeRouter),
    }
}

pub fn require_xlm(e: &Env) -> Address {
    match read_xlm(e) {
        Some(xlm) => xlm,
        None => panic_with_error!(&e, Error::RequiresXlmID),
    }
}
