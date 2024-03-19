use soroban_sdk::{contractclient, contractspecfn, contracttype, Address, Symbol, Val, Vec};

pub use IExchange as Interface;
pub use IExchangeClient as Client;

#[contracttype]
pub struct CallData {
    pub function: Symbol,
    pub data: Vec<Val>,
}

#[contractclient(name = "IExchangeClient")]
#[contractspecfn(name = "IExchangeSpec", export = false)]
pub trait IExchange {
    fn get_call_data(
        token_in_id: Address,
        token_out_id: Address,
        amount_in: i128,
        amount_out: i128,
        to: Address,
        deadline: u64,
    ) -> CallData;
}

/// Spec contains the contract spec of iExchange contracts, including the general
/// interface, as well as the admin interface, such as the Stellar Asset
/// Contract.
#[doc(hidden)]
pub struct IExchangeSpec;

pub(crate) const SPEC_XDR_INPUT: &[&[u8]] = &[&IExchangeSpec::spec_xdr_get_call_data()];

pub(crate) const SPEC_XDR_LEN: usize = 5336;

impl IExchangeSpec {
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
