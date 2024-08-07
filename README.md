# About
 Rust wallet is a smart Wallet build on the Internet Computer Blockchain. The wallet has a functionalities of Sending and Receiving tokens.

 #### Deploy Locally
```
dfx deploy
```
#### Test Wallet Functions

Account Creation For Alice and Bob (Returns True)
```
dfx canister call rust_wallet_backend create_account '("Alice")'
dfx canister call rust_wallet_backend create_account '("Bob")'
```
Check Balances
```
dfx canister call rust_wallet_backend get_balance '("Alice")'
```
Run Tests
```
cargo test
```

