
cd ./solana_programs_riscv
export SOLANA_PROGRAMS_RISCV=`pwd`
cd -

cd ../../cartesi-solana
export CARTESI_SOLANA=`pwd`
cd -

docker run \
    -v `pwd`:/workdir \
    -v $CARTESI_SOLANA:/cartesi-solana \
    -v $SOLANA_PROGRAMS_RISCV:/solana_programs_riscv \
    -w /workdir \
    --rm \
    cartesi/toolchain:0.11.0 \
    ./build-prod.sh


