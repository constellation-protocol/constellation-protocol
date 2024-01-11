use soroban_sdk::{
    contract, contracterror, contractimpl, log, panic_with_error, symbol_short, token, Address,
    Env, String, Symbol, Val, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ZeroValue = 100,
    ExceedsMaxComponents = 101,
}
