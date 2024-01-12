use soroban_sdk::{Address, Env};
use super::{INSTANCE_LEDGER_LIFE, INSTANCE_LEDGER_TTL_THRESHOLD};

use super::DataKey;

pub(crate) fn extend_ttl(e: &Env) {
   e.storage().instance().extend_ttl(INSTANCE_LEDGER_TTL_THRESHOLD, INSTANCE_LEDGER_LIFE);
}

pub(crate) fn read_max_components(e: &Env) -> Option<u32> {
    extend_ttl(e);
    let key = DataKey::MaxComponents;
    e.storage().persistent().get(&key)
}

pub(crate) fn write_max_components(e: &Env, val: u32) {
    extend_ttl(e);
    let key = DataKey::MaxComponents;
    e.storage().persistent().set(&key, &val)
}
