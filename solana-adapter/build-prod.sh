
set -e
cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ./riscv64ima-cartesi-linux-gnu.json --release
cp ./target/riscv64ima-cartesi-linux-gnu/release/solana_adapter ./solana_adapter
echo "done."
