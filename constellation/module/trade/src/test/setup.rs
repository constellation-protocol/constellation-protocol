extern crate std;

use super::add_liquidity::add_liquidity;
use super::clients::{
    create_adapter, create_constellation_token, create_registry, create_soroswap_factory,
    create_soroswap_router, create_token_contract, create_trade_module, pair_contract_wasm,
    registry, ConstellationTokenClient, RegistryClient, SoroswapFactoryClient,
    SoroswapRouterClient, TokenClient, TradeAdapterClient,
};
use crate::contract::TradeClient;
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String, Vec};
pub type Tokens<'a> = (
    TokenClient<'a>,
    TokenClient<'a>,
    TokenClient<'a>,
    TokenClient<'a>,
);

pub struct TradeTest<'a> {
    pub env: Env,
    pub user: Address,
    pub admin: Address,
    pub adapter: TradeAdapterClient<'a>,
    pub registry: RegistryClient<'a>,
    pub router: SoroswapRouterClient<'a>,
    pub factory: SoroswapFactoryClient<'a>,
    pub constellation_token: ConstellationTokenClient<'a>,
    pub trade_module: TradeClient<'a>,
    pub tokens: Tokens<'a>,
}

impl<'a> TradeTest<'a> {
    pub fn setup() -> TradeTest<'a> {
        let env = Env::default();
        env.mock_all_auths();
        let user = Address::generate(&env);
        let admin = Address::generate(&env);
        let adapter = create_adapter(&env);
        let router = create_soroswap_router(&env);
        let factory = create_soroswap_factory(&env);
        let constellation_token = create_constellation_token(&env);
        let registry = create_registry(&env);
        let trade_module = create_trade_module(&env);

        adapter.initialize(&router.address, &factory.address);
        router.initialize(&factory.address);
        registry.initialize(&admin);
        factory.initialize(&admin, &pair_contract_wasm(&env));
        trade_module.initialize(&registry.address);

        let mut tokens: Tokens = (
            create_token_contract(&env, &admin),
            create_token_contract(&env, &admin),
            create_token_contract(&env, &admin),
            create_token_contract(&env, &admin),
        );

        // if &tokens.1.address < &tokens.0.address {
        //     std::mem::swap(&mut tokens.0, &mut tokens.1);
        // }

        tokens.0.mint(&user, &10_000_000_000_000_000_000);
        tokens.1.mint(&user, &10_000_000_000_000_000_000);
        tokens.2.mint(&user, &10_000_000_000_000_000_000);
        tokens.3.mint(&user, &10_000_000_000_000_000_000);

        let amount_0: i128 = 4_000_000_000;
        let amount_1: i128 = 4_000_000_000;

        add_liquidity(
            &env,
            &router,
            &user,
            &tokens.0.address,
            &tokens.1.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &router,
            &user,
            &tokens.2.address,
            &tokens.3.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &router,
            &user,
            &tokens.0.address,
            &tokens.2.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &router,
            &user,
            &tokens.0.address,
            &tokens.3.address,
            &amount_0,
            &amount_1,
        );

        env.budget().reset_unlimited();

        Self {
            env,
            user,
            admin,
            adapter,
            router,
            registry,
            factory,
            constellation_token,
            tokens,
            trade_module,
        }
    }
}
