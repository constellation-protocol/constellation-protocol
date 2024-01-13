
# Demo CLI Walkthrough

## In this Demo we will demonstrate the following

- Mint Constellation Tokens to Account A
- Transfer Constellation token to account B
- Burn Constellation Tokens from Account B so that Account B receives the component tokens

## To achieve the above , we will

- Deploy 3 component tokens namely USDC, UNI, AAVE
- Install soroban Freighter wallet chrome extension so that we can view balances
- Create 2 accounts from Soroban Cli
- Add both accounts to Freight Wallet so we can view their balances
- Add Constellation token and component token address to Freight wallet so we can view users balance

## Initial Preparation

- Install Soroban CLI
- Initialize your account using the cli
- Install [Freighter wallet](https://www.freighter.app) chrome extension. This will be used to view your tokens
- Add your local soroban account to Freighter wallet
  1. Obtain the account private key by:
   `
    soroban config identity show
   `
  2. Copy the revealed private key
  3. Add the private key to Freigher wallet. Open Freighter wallet extension which is installed on your chrome, click on the account icon which is at the top left of the wallet app. At the bottom, click 'Import a stellar secret key' and paste your local account secret key

## Run The Following commands in the root of the project

## Install & Deploy Component Tokens

### 1. Install Component Token

`
soroban contract install --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm libs/soroban_token_contract.wasm
`

### 2. Deploy Component token contracts (USDC, UNI , AAVE)

#### i. Deploy USDC Token

`
soroban contract deploy --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm-hash REPLACE_WITH_INSTALLED_TOKEN_CONTRACT_HASH
`

#### ii Deploy UNI Token

`
soroban contract deploy --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm-hash REPLACE_WITH_INSTALLED_TOKEN_CONTRACT_HASH
`

#### iii Deploy AAVE Token

`
soroban contract deploy --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm-hash REPLACE_WITH_INSTALLED_TOKEN_CONTRACT_HASH
`

### 3. Initialze Component Tokens

#### i. Initialize USDC

`
soroban contract invoke --id REPLACE_WITH_INSTALLED_USDC_CONTRACT_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- initialize --admin REPLACE_WITH_YOUR_ACCOUNT_ADDRESS --decimal 6 --name "USDC" --symbol "USDC"
`

#### ii. Initialize UNI

`
soroban contract invoke --id REPLACE_WITH_DEPLOYED_UNI_CONTRACT_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- initialize --admin REPLACE_WITH_YOUR_ACCOUNT_ADDRESS --decimal 6 --name "Uniswap" --symbol "UNI"
`

#### iii. Initialize AAVE

`
soroban contract invoke --id REPLACE_WITH_DEPLOYED_AAVE_CONTRACT_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- initialize --admin REPLACE_WITH_YOUR_ACCOUNT_ADDRESS --decimal 6 --name "AAVE" --symbol "AAVE"
`

### 4 Mint Component Tokens to your account

#### Mint 500 million (500000000000000)  to your account - Repeat for all component tokens

`
soroban contract invoke --id REPLACE_WITH_DEPLOYED_TOKEN_CONTRACT_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- mint --to REPLACE_WITH_YOUR_ACCOUNT_ADDRESS --amount 500000000000000
`

### 5. Install & Deploy Factory

#### i. Install Factory

`
soroban contract install --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm target/wasm32-unknown-unknown/release/constellation_factory.wasm
`

#### ii. Deploy Factory

`
soroban contract deploy --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm-hash REPLACE_WITH_INSTALLED_FACTORY_HASH
`

### 6. Install & Deploy Router

#### i. Install Router

`
soroban contract install --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm target/wasm32-unknown-unknown/release/constellation_router.wasm
`

#### ii. Deploy Router

`
soroban contract deploy --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm-hash REPLACE_WITH_INSTALLED_ROUTER_HASH
`


#### iii. Initialize Router

`
soroban contract invoke --id REPLACE_WITH_DEPLOYED_ROUTER_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- initialize --factory REPLACE_WITH_FACTORY_CONTRACT_ADDRESS
`

### 6. Install & Deploy Constellation Token

#### i. Install Constellation Token

`
soroban contract install --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm target/wasm32-unknown-unknown/release/constellation_token.wasm
`

#### ii. Deploy Constellation Token

`
soroban contract deploy --source REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet --wasm-hash REPLACE_WITH_CONSTELLATION_TOKEN_CONTRACT_ADDRESS
`

#### iii. Initialize Constellation Token

`
soroban contract invoke --id REPLACE_WITH_CONSTELLATION_TOKEN_CONTRACT_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- initialize --decimal 6 --components '["REPLACE_WITH_USDC_TOKEN_ADDRESS", "REPLACE_WITH_UNI_TOKEN_ADDRESS", "REPLACE_WITH_AAVE_TOKEN_ADDRESS"]' --amounts '["1000", "500", "300"]' --name "USDC-UNI-AAVE" --symbol "UUA" --admin REPLACE_WITH_ROUTER_ADDRESS --manager REPLACE_WITH_YOUR_ACCOUNT_ADDRESS
`

### 7. Mint Constellation Token

#### i. Approve (repeat for all component tokens)

`
soroban contract invoke --id REPLACE_WITH_COMPONENT_TOKEN_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- approve --from REPLACE_WITH_YOUR_ACCOUNT_ADDRESS  --spender REPLACE_WITH_CONSTELLATION_TOKEN_ADDRESS --amount 10000000000
`

#### ii. Mint

`
soroban contract invoke --id REPLACE_WITH_ROUTER_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- mint --to REPLACE_WITH_RECEIVING_ADDRESS  --constellation_token_address REPLACE_WITH_CONSTELLATION_TOKEN_ADDRESS --amount 10000000
`

### 8. Transfer Constellation Token to Another Account B

  soroban contract invoke --id REPLACE_WITH_CONSTELLATION_TOKEN_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- transfer --from REPLACE_WITH_YOUR_ACCOUNT_ADDRESS --to REPLACE_WITH_RECEIVING_ACCOUNT_ADDRESS --amount 2000000

### 9. Burn Constellation Token to redeem Assets from Account B

#### i Approve Router

soroban contract invoke --id REPLACE_WITH_CONSTELLATION_TOKEN_ADDRESS  --source-account REPLACE_WITH_SENDER_ACCOUNT_NAME --network testnet -- approve --from REPLACE_WITH_SENDER_ACCOUNT_ADDRESS  --spender REPLACE_WITH_ROUTER_ADDRESS --amount 10000000000 --expiration_ledger 100000

#### i Redeem

`
soroban contract invoke --id REPLACE_WITH_ROUTER_ADDRESS --source-account REPLACE_WITH_YOUR_ACCOUNT_NAME --network testnet -- burn --from REPLACE_WITH_YOUR_ACCOUNT_ADDRESS  --constellation_token_address REPLACE_WITH_CONSTELLATION_TOKEN_ADDRESS --amount AMOUNT_TO_BURN
`
