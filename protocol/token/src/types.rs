use soroban_sdk::{contracttype, Address};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const COMPONENTS_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const COMPONENTS_LIFETIME_THRESHOLD: u32 = COMPONENTS_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone, Debug)]
#[contracttype]
pub struct Component {
    pub address: Address,
    pub amount: i128,
}

#[derive(Clone)]
#[contracttype]
pub(crate) enum DataKey {
    Components,
    Manager,
}
