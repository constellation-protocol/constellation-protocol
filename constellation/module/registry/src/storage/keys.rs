use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Adapter(Address /*module_id*/, Address /* adapter_id*/),
    Module(Address),
}
