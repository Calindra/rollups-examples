
set -e
cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ./riscv64ima-cartesi-linux-gnu.json --release
cargo clean
echo "done."
