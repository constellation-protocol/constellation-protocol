use soroban_sdk::{
    auth::InvokerContractAuthEntry, contractclient, contractspecfn, contracttype, Address, Env,
    Symbol, Val, Vec,
};

pub use IExchange as Interface;
pub use IExchangeClient as Client;

#[contractclient(name = "IExchangeClient")]
#[contractspecfn(name = "IExchangeSpec", export = false)]
pub trait IExchange {
    fn get_swap_call_data(
        e: &Env,
        token_in_id: Address,
        token_out_id: Address,
        amount_in: i128,
        amount_out: i128,
        to: Address,
        deadline: u64,
    ) -> (Symbol, Vec<Val>);

    fn get_approve_call_data(
        e: &Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) -> (Symbol, Vec<Val>);

    fn create_sub_auth(
        e: &Env,
        amount_in: i128,
        token_in: Address,
        token_out: Address,
        constellation_token_id: Address,
    ) -> Vec<InvokerContractAuthEntry>;
}

/// Spec contains the contract spec of iExchange contracts, including the general
/// interface, as well as the admin interface, such as the Stellar Asset
/// Contract.
#[doc(hidden)]
pub struct IExchangeSpec;

pub(crate) const SPEC_XDR_INPUT: &[&[u8]] = &[
    &IExchangeSpec::spec_xdr_get_swap_call_data(),
    &IExchangeSpec::spec_xdr_get_approve_call_data(),
    &IExchangeSpec::spec_xdr_create_sub_auth(),
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
