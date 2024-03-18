use crate::error::Error;
use soroban_sdk::{contractclient, contractspecfn, Address, BytesN, Env, String, Vec};

pub use IFactory as Interface;
pub use IFactoryClient as Client;

#[contractclient(name = "IFactoryClient")]
#[contractspecfn(name = "IFactorySpec", export = false)]
pub trait IFactory {
    fn initialize(e: Env, admin: Address) -> Result<(), Error>;
    #[allow(clippy::too_many_arguments)]
    fn create(
        e: Env,
        decimal: u32,
        name: String,
        symbol: String,
        admin: Address,
        manager: Address,
        components: Vec<Address>,
        amounts: Vec<i128>,
        deployer: Address,
        wasm_hash: BytesN<32>,
        salt: BytesN<32>,
    ) -> Result<Address, Error>;
    fn set_max_components(e: Env, value: u32) -> Result<(), Error>;
    fn get_token_list(e: Env) -> Vec<Address>;
    fn get_max_components(e: Env) -> Option<u32>;
}

/// Spec contains the contract spec of iExchange contracts, including the general
/// interface, as well as the admin interface, such as the Stellar Asset
/// Contract.
#[doc(hidden)]
pub struct IFactorySpec;

pub(crate) const SPEC_XDR_INPUT: &[&[u8]] = &[
    &IFactorySpec::spec_xdr_create(),
    &IFactorySpec::spec_xdr_set_max_components(),
    &IFactorySpec::spec_xdr_get_token_list(),
    &IFactorySpec::spec_xdr_get_max_components(),
];

pub(crate) const SPEC_XDR_LEN: usize = 5336;

impl IFactorySpec {
    /// Returns the XDR spec for the Token contract.
    pub const fn spec_xdr() -> [u8; SPEC_XDR_LEN] {
        let input = SPEC_XDR_INPUT;
        // Concatenate all XDR for each item that makes up the imm spec.
        let mut output = [0u8; SPEC_XDR_LEN];
        let mut input_i = 0;
        let mut output_i = 0;
        while input_i < input.len() {
            let subinput = input[input_i];
            let mut subinput_i = 0;
            while subinput_i < subinput.len() {
                output[output_i] = subinput[subinput_i];
                output_i += 1;
                subinput_i += 1;
            }
            input_i += 1;
        }

        // Check that the numbers of bytes written is equal to the number of bytes
        // expected in the output.
        if output_i != output.len() {
            panic!("unexpected output length",);
        }

        output
    }
}
