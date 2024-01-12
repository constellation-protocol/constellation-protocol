use super::{INSTANCE_LEDGER_LIFE, INSTANCE_LEDGER_TTL_THRESHOLD};
use soroban_sdk::{Address, Env};

use super::DataKey;

pub(crate) fn extend_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LEDGER_TTL_THRESHOLD, INSTANCE_LEDGER_LIFE);
}

pub(crate) fn write_factory(e: &Env, factory: &Address) {
    e.storage().instance().set(&DataKey::Factory, factory);
    extend_ttl(e);
}

pub(crate) fn read_factory(e: &Env) -> Address {
    extend_ttl(e);
    e.storage().instance().get(&DataKey::Factory).unwrap()
}

pub(crate) fn has_factory(e: &Env) -> bool {
    extend_ttl(e);
    e.storage().instance().has(&DataKey::Factory)
}
