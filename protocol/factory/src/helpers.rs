// Import necessary types from the Soroban SDK
// #![allow(unused)]
use soroban_sdk::{contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env};

pub(crate) fn deploy(
    e: &Env,
    deployer: Address,
    wasm_hash: BytesN<32>,
    salt: BytesN<32>,
) -> Address {
    // Skip authorization if deployer is the current contract.
    if deployer != e.current_contract_address() {
        deployer.require_auth();
    }

    // Deploy the contract using the uploaded Wasm with given hash.
    e.deployer().with_address(deployer, salt).deploy(wasm_hash)
}
