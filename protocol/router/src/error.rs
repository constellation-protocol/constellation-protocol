use soroban_sdk::{
    contract, contracterror, contractimpl, log, symbol_short, token, Address, Env, InvokeError,
    String, Symbol, Val, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    ZeroOrNegativeAmount = 401,
    InsufficientAllowance = 500,
    InsufficientBalance = 501,
    ContractInvocationError = 502,
    AlreadyInitalized = 503,
    RequiresFactory = 504,
}
