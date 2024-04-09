#![cfg(test)]
extern crate std;
use super::router::create_soroswap_router;
use super::router::SoroswapRouterClient;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../../libs/soroban_token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;

pub fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

// Pair Contract
mod pair {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_pair.wasm");
    pub type SoroswapPairClient<'a> = Client<'a>;
}
use pair::SoroswapPairClient;

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_pair.wasm");
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../../libs/soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

fn create_soroswap_factory<'a>(e: &Env, setter: &Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter TEST

pub struct SoroswapRouterTest<'a> {
    pub env: Env,
    pub contract: SoroswapRouterClient<'a>,
    pub token_0: TokenClient<'a>,
    pub token_1: TokenClient<'a>,
    pub factory: SoroswapFactoryClient<'a>,
    pub user: Address,
    pub admin: Address,
}

impl<'a> SoroswapRouterTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_router(&env);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(admin, user);

        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        
        if &token_1.address < &token_0.address {
            std::mem::swap(&mut token_0, &mut token_1);
        }
        token_0.mint(&user, &10_000_000_000_000_000_000);
        token_1.mint(&user, &10_000_000_000_000_000_000);

        let factory = create_soroswap_factory(&env, &admin);
        env.budget().reset_unlimited();

        SoroswapRouterTest {
            env,
            contract,
            token_0,
            token_1,
            factory,
            user,
            admin,
        }
    }
 
    fn setup_deducted_reserve() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_router(&env);

        let admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(admin, user);

        let mut token_0 = create_token_contract(&env, &admin);
        let mut token_1 = create_token_contract(&env, &admin);
        if &token_1.address < &token_0.address {
            std::mem::swap(&mut token_0, &mut token_1);
        }

        let initial_user_balance = 24_995_705_032_704;

        token_0.mint(&user, &initial_user_balance);
        token_1.mint(&user, &initial_user_balance);

        let factory = create_soroswap_factory(&env, &admin);

        SoroswapRouterTest {
            env,
            contract,
            token_0,
            token_1,
            factory,
            user,
            admin,
        }
    }
}

// // Test mods:

// pub mod initialize;
// pub mod add_liquidity;
// //pub mod swap;
// pub mod remove_liquidity;
// pub mod library_functions;
// pub mod swap_tokens_for_exact_tokens;
// pub mod swap_exact_tokens_for_tokens;
// pub mod events;

// // BUDGET TEST MOD
// mod budget;
