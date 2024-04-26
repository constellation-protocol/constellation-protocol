use super::{INSTANCE_LEDGER_LIFE, INSTANCE_LEDGER_TTL_THRESHOLD};
use soroban_sdk::{Address, Env};

use super::DataKey;

pub(crate) fn extend_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LEDGER_TTL_THRESHOLD, INSTANCE_LEDGER_LIFE);
}

pub(crate) fn write_xlm(e: &Env, xml: &Address) {
    e.storage().instance().set(&DataKey::XLM, xml);
    extend_ttl(e);
}

pub(crate) fn read_xlm(e: &Env) -> Option<Address> {
    extend_ttl(e);
    e.storage().instance().get(&DataKey::XLM)
}
