use soroban_sdk::{
    contract, contracterror, contractimpl, log, panic_with_error, symbol_short, token, Address,
    Env, String, Symbol, Val, Vec,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Errors caused by invalid user unput
    ///
    // expiration_ledger is less than ledger seq when amount is greater than zero
    ExpirationLedgerLessThanLedgerSequence = 400,
    ZeroOrNegativeAmount = 401,
    NegativeAmount = 402,
    ZeroAmount = 403,
    ZeroComponents = 404,
    ZeroLength = 405,
    IndexUnwrapError = 406,
    ComponentsAmountsLengthMismatch = 407,
    ValueTooLargeOverFlow = 408,

    /// Errors caused by smart contract state or logic
    InsufficientAllowance = 500,
    InsufficientBalance = 501,
    AlreadyInitalized = 502,
    RequiresAdministrator = 503,
    RequiresManager = 504,
    RequiresRegistry = 505,
    RequiresManage = 506,
    RequiresRegisteredModule = 507,
    UnregisteredModule = 508,
}

pub fn check_zero_or_negative_amount(e: &Env, amount: i128) {
    if amount <= 0 {
        panic_with_error!(&e, Error::ZeroOrNegativeAmount);
    }
}

pub fn check_nonnegative_amount(e: &Env, amount: i128) {
    if amount < 0 {
        panic_with_error!(&e, Error::NegativeAmount);
    }
}
