export SOLANA_DATA_PATH="./account_info_data"
export ROLLUP_HTTP_SERVER_URL="http://127.0.0.1:5004"
export RUST_BACKTRACE=1
export PORTAL_ADDRESS=0xf8c694fd58360de278d5ff2276b7130bfdc0192a

cargo watch -i account_info_data -x 'run'
