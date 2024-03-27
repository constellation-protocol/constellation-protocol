use soroban_sdk::{contracttype, Address};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const PERSISTENT_LEDGER_LIFE: u32 = 90 * DAY_IN_LEDGERS; // ~90 days.
pub(crate) const PERSISTENT_LEDGER_TTL_THRESHOLD: u32 = PERSISTENT_LEDGER_LIFE - DAY_IN_LEDGERS;

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Component {
    pub address: Address,
    pub unit: i128,
}