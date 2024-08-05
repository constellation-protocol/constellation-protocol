use crate::error::Error;
use crate::storage::constellation_token_hash::read_constellation_hash;
use soroban_sdk::{BytesN, Env};
pub fn require_constellation_wasm_hash(e: &Env) -> Result<BytesN<32>, Error> {
    match read_constellation_hash(&e) {
        Some(hash) => Ok(hash),
        None => return Err(Error::ReqiuresConstellationWasmTokenHash),
    }
}
