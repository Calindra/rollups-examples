
set -e
cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ./riscv64ima-cartesi-linux-gnu.json --release
cp ./target/riscv64ima-cartesi-linux-gnu/release/base_spl ../solana_programs_riscv/11111111111111111111111111111111
cargo clean
echo "done."
