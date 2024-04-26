extern crate std;

use super::add_liquidity::add_liquidity;
use super::clients::{
    create_adapter, create_constellation_token, create_registry, create_router,
    create_soroswap_factory, create_soroswap_router, create_token_contract, pair_contract_wasm,
    registry, ConstellationTokenClient, RegistryClient, RouterClient, SoroswapFactoryClient,
    SoroswapRouterClient, TokenClient, TradeAdapterClient,
};

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
    pub router: RouterClient<'a>,
    pub s_router: SoroswapRouterClient<'a>,
    pub factory: SoroswapFactoryClient<'a>,
    pub constellation_token: ConstellationTokenClient<'a>,
    pub tokens: Tokens<'a>,
    pub deadline: u64,
}

impl<'a> TradeTest<'a> {
    pub fn setup() -> TradeTest<'a> {
        let env = Env::default();
        env.mock_all_auths();
        let user = Address::generate(&env);
        let admin = Address::generate(&env);
        let adapter = create_adapter(&env);
        let s_router = create_soroswap_router(&env);
        let factory = create_soroswap_factory(&env);
        let constellation_token = create_constellation_token(&env);
        let registry = create_registry(&env);
        let router = create_router(&env);

        // constellation_token.initialize(decimal, components, units, name, symbol, &admin, manager);

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

        adapter.initialize(&s_router.address, &factory.address);
        s_router.initialize(&factory.address);
        registry.initialize(&admin);
        factory.initialize(&admin, &pair_contract_wasm(&env));
        router.initialize(&factory.address, &s_router.address, &tokens.0.address);

        let amount_0: i128 = 4_000_000_000;
        let amount_1: i128 = 4_000_000_000;

        add_liquidity(
            &env,
            &s_router,
            &user,
            &tokens.0.address,
            &tokens.1.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &s_router,
            &user,
            &tokens.2.address,
            &tokens.3.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &s_router,
            &user,
            &tokens.0.address,
            &tokens.2.address,
            &amount_0,
            &amount_1,
        );

        add_liquidity(
            &env,
            &s_router,
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
            s_router,
            router,
            registry,
            factory,
            constellation_token,
            tokens,
            deadline: 10000000u64,
        }
    }
}
