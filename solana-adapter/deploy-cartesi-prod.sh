
cd ./solana_programs_riscv
export SOLANA_PROGRAMS_RISCV=`pwd`
cd -

docker run \
    -v `pwd`:/workdir \
    -v $SOLANA_PROGRAMS_RISCV:/solana_programs_riscv \
    -w /workdir \
    --rm \
    cartesi/toolchain:0.11.0 \
    ./build-prod.sh


