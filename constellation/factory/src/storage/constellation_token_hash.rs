use super::{INSTANCE_LEDGER_LIFE, INSTANCE_LEDGER_TTL_THRESHOLD};
use soroban_sdk::{Address, BytesN, Env};

use super::DataKey;

pub(crate) fn extend_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LEDGER_TTL_THRESHOLD, INSTANCE_LEDGER_LIFE);
}
pub(crate) fn read_constellation_hash(e: &Env) -> Option<BytesN<32>> {
    extend_ttl(e);
    let key = DataKey::ConstellationTokenHash;
    e.storage().persistent().get(&key)
}

pub(crate) fn write_constellation_hash(e: &Env, val: &BytesN<32>) {
    extend_ttl(e);
    let key = DataKey::ConstellationTokenHash;
    e.storage().persistent().set(&key, val)
}
