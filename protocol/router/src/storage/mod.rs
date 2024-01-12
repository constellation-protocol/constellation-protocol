use soroban_sdk::contracttype;

pub(crate) mod factory;
pub(crate) use factory::{has_factory, read_factory, write_factory};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_LEDGER_LIFE: u32 = 30 * DAY_IN_LEDGERS; // ~30 days.
pub(crate) const INSTANCE_LEDGER_TTL_THRESHOLD: u32 = INSTANCE_LEDGER_LIFE - DAY_IN_LEDGERS;

pub(crate) const PERSISTENT_LEDGER_LIFE: u32 = 90 * DAY_IN_LEDGERS; // ~90 days.
pub(crate) const PERSISTENT_LEDGER_TTL_THRESHOLD: u32 = PERSISTENT_LEDGER_LIFE - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Factory,
}
