# just compile and copy the binary to folder
cd solana-smart-contract && \ 
    cargo build && \
    cp ./target/debug/solana_smart_contract ../solana_smart_contract_bin/2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv

# basic functions
cd ../base_spl && cargo build
cp ./target/debug/base_spl ../solana_smart_contract_bin/11111111111111111111111111111111
