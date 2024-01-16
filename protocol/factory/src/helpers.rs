use soroban_sdk::{contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env};

pub(crate) fn deploy(
    e: &Env,
    deployer: Address,
    wasm_hash: BytesN<32>,
    salt: BytesN<32>,
) -> Address {
    // Deploy the contract using the uploaded Wasm with given hash.
    e.deployer().with_address(deployer, salt).deploy(wasm_hash)
}
