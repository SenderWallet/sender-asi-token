# SAI TOKEN

#### 📦 Dependencies

- Install near-cli: `npm install -g near-cli`
- Install Rust (>= 1.60.0) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- `rustup target add wasm32-unknown-unknown`

#### Build & Run tests

```rust
./scripts/build.sh
//The wasm file will be at `res/sender.wasm`

cargo test -- --nocapture
cargo run --example mint
cargo run --example transfer
```

## Usage

```bash
export TOKEN_ACCOUNT_ID=your-token-account-id
```

Deploy and initialize the contract:

```bash
near deploy --accountId=$TOKEN_ACCOUNT_ID --wasmFile=res/sender.wasm --initArgs '{"owner":"'$OWNER'","total_supply":"100000000000000000000000000"}' --initFunction new
```

Call view methods

```bash
near view $TOKEN_ACCOUNT_ID ft_balance_of '{"account_id":"some-random-account.testnet"}'
'0'
```

```

Transfer tokens

```bash
# not necessarily $ORACLE_ACCOUNT_ID, can be any local account
near call $TOKEN_ACCOUNT_ID ft_transfer '{"receiver_id":"<receiver id>", "amount":"100", "memo":"hello world!"}' --accountId $ORACLE_ACCOUNT_ID --depositYocto 1
```

Pay for storage

```bash
# not necessarily $ORACLE_ACCOUNT_ID, can be any local account
near call $TOKEN_ACCOUNT_ID storage_deposit '{"account_id":"random-guy-1.testnet"}' --accountId $ORACLE_ACCOUNT_ID --depositYocto 2350000000000000000000

near view $TOKEN_ACCOUNT_ID storage_balance_of '{"account_id":"random-guy-1.testnet"}' --accountId $ORACLE_ACCOUNT_ID
```

Mint tokens

```bash
near call $TOKEN_ACCOUNT_ID mint_tge '{"amount":"100", "account_for":"<account_for>"}' --accountId $TOKEN_ACCOUNT_ID --gas=300000000000000
```

### Deferring tokens

![Smart contracts interaction](doc/contracts_interaction.png)

For information about claiming tokens from Holding account refer to the [Sweat Claim repo](https://github.com/sweatco/sweat-claim). 
