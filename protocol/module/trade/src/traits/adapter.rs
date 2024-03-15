use soroban_sdk::{Address,Symbol, Vec, Val,contracttype, contractclient, contractspecfn };


pub use IExchangeClient as Client;
pub use IExchange as Interface;

#[contracttype]
pub struct CallData {
    target: Address,
    method: Symbol,
    data: Vec<Val>,
}

#[contractclient(name="IExchangeClient")]
#[contractspecfn(name = "IExchangeSpec")]
pub trait IExchange {
    fn get_call_data() ->CallData ;
}

/// Spec contains the contract spec of iExchange contracts, including the general
/// interface, as well as the admin interface, such as the Stellar Asset
/// Contract.
#[doc(hidden)]
pub struct  IExchangeSpec;

pub(crate) const SPEC_XDR_INPUT: &[&[u8]] = &[ 
    &IExchangeSpec::spec_xdr_get_call_data(),
];

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
