use soroban_sdk::contracttype;

pub(crate) mod admin;
pub(crate) mod max_components;
pub(crate) mod token_list;
pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_LEDGER_LIFE: u32 = 30 * DAY_IN_LEDGERS; // ~30 days.
pub(crate) const INSTANCE_LEDGER_TTL_THRESHOLD: u32 = INSTANCE_LEDGER_LIFE - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    MaxComponents,
    TokenList,
}
