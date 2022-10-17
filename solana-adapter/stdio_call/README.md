
The simplest way to execute a program

```
cargo build

export SOLANA_DATA_PATH=./data
cat ./tests/fixtures/create_account_in.txt | ./target/debug/stdio_call
```

```
cargo build --release

export SOLANA_DATA_PATH=./data
cat ./tests/fixtures/create_account_in.txt | ./target/release/stdio_call
```