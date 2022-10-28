# syntax=docker.io/docker/dockerfile:1.4
FROM cartesi/toolchain:0.11.0 as dapp-build

WORKDIR /opt/cartesi/dapp

COPY . .

RUN cd ./base_spl && cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ../riscv64ima-cartesi-linux-gnu.json --release
RUN cp /opt/cartesi/dapp/base_spl/target/riscv64ima-cartesi-linux-gnu/release/base_spl /opt/cartesi/dapp/solana_smart_contract_bin/11111111111111111111111111111111

RUN cd ./solana-smart-contract && cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target ../riscv64ima-cartesi-linux-gnu.json --release
RUN cp /opt/cartesi/dapp/solana-smart-contract/target/riscv64ima-cartesi-linux-gnu/release/solana_smart_contract /opt/cartesi/dapp/solana_smart_contract_bin/2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv
RUN cargo build -Z build-std=std,core,alloc,panic_abort,proc_macro --target riscv64ima-cartesi-linux-gnu.json --release
