# Solana Adapter

This example simulates a Solana environment so you can run your smart contract on top of a compatible EVM network.

## Host Mode

Compile your program and place it inside the folder `solana_smart_contract_bin`.

## Prod Mode

Programs need to be compiled for the Cartesi Machine RISC-V architecture and be placed in the `solana_programs_riscv` folder.

## Solana State

Solana's data is saved inside the `account_info_data` folder.

AirDrop account:  
account_info_data/9B5XszUGdMaxCZ7uSQhPzdks5ZQSmWxrmzCSvtJ6Ns6g.json

## Developer side notes

How to run tests

```shell
cargo watch -x 'test -- --nocapture --test-threads 1'
```

How to clean the accounts folder

```shell
git clean -f account_info_data/
```

How to reclaim docker disk space

```shell
docker system prune -a --volumes
```

Rust nightly version to expand macros

```shell
rustup default nightly
```
