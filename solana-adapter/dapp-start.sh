export SOLANA_DATA_PATH="./account_info_data"
export ROLLUP_HTTP_SERVER_URL="http://127.0.0.1:5004"
export RUST_BACKTRACE=1
cargo watch -i account_info_data -x 'run'
