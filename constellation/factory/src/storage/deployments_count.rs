use super::{INSTANCE_LEDGER_LIFE, INSTANCE_LEDGER_TTL_THRESHOLD};
use soroban_sdk::{Address, Env};

use super::DataKey;

pub(crate) fn extend_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LEDGER_TTL_THRESHOLD, INSTANCE_LEDGER_LIFE);
}
pub(crate) fn read_deployment_count(e: &Env) -> Option<u64> {
    extend_ttl(e);
    let key = DataKey::DeploymentCount;
    e.storage().persistent().get(&key)
}

pub(crate) fn write_deployment_count(e: &Env, val: u64) {
    extend_ttl(e);
    let key = DataKey::DeploymentCount;
    e.storage().persistent().set(&key, &val)
}
