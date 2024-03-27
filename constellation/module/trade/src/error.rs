use soroban_sdk::{contracterror, contractimpl};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitalized = 101,
    UnregisteredAdapter = 102,
    RequiresAdmin = 103,
    RequiresRegistry = 104,
    RequiresExchangeAdapter = 105,
    RequiresManage = 106,
}
