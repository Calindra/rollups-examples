# syntax=docker.io/docker/dockerfile:1.4
FROM cartesi/toolchain:0.11.0 as dapp-build

WORKDIR /opt/cartesi/dapp

COPY . .

# Compile the system program
# RUN cd ./base_spl && cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ../riscv64ima-cartesi-linux-gnu.json --release
# RUN cp /opt/cartesi/dapp/base_spl/target/riscv64ima-cartesi-linux-gnu/release/base_spl /opt/cartesi/dapp/solana_smart_contract_bin/11111111111111111111111111111111

# RUN cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target riscv64ima-cartesi-linux-gnu.json --release
