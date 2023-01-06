use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

use cartesi_solana::{owner_manager, adapter::eth_address_to_pubkey};
use solana_adapter::{deposit, inspect};
use solana_program::pubkey::Pubkey;

fn setup() {
    println!("\n\n***** setup *****\n");
    let dir = std::env::temp_dir();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let final_temp_dir = format!(
        "{}/{}",
        dir.as_os_str().to_str().unwrap(),
        since_the_epoch.subsec_nanos()
    );
    println!("{}", final_temp_dir);
    fs::create_dir(&final_temp_dir).unwrap();
    std::env::set_var("SOLANA_DATA_PATH", final_temp_dir);
    deposit::only_accepts_deposits_from_address(
        "0xf8c694fd58360de278d5ff2276b7130bfdc0192a".to_string(),
    );
    unsafe {
        owner_manager::POINTERS.clear();
        owner_manager::OWNERS.clear();
    }
}

fn create_token_account(depositor: &str) -> Pubkey {
    let payload = format!("0x59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378000000000000000000000000{}00000000000000000000000067d269191c92caf3cd7723f116c85e6e9bf5593300000000000000000000000000000000000000000000000000000000000004d200000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000", depositor);
    let msg_sender = "0xf8c694fd58360de278d5ff2276b7130bfdc0192a";
    let timestamp = "1";
    deposit::process(&payload, &msg_sender, &timestamp.to_string())
}

fn create_token_account_with_mint(depositor: &str, mint: &str) -> Pubkey {
    let payload = format!("0x59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378000000000000000000000000{}000000000000000000000000{}00000000000000000000000000000000000000000000000000000000000004d200000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000", depositor, mint);
    let msg_sender = "0xf8c694fd58360de278d5ff2276b7130bfdc0192a";
    let timestamp = "1";
    deposit::process(&payload, &msg_sender, &timestamp.to_string())
}

#[test]
fn it_should_find_program_accounts() {
    setup();
    create_token_account("f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    create_token_account("B2FeF42289bBdea2d1E3d712202b3Ca2C48eA989");
    let accounts =
        inspect::find_program_accounts("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    assert_eq!(accounts.len(), 2);
}

#[test]
fn it_should_read_account_info() {
    setup();
    let pubkey = create_token_account("f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    let account_info = inspect::read_account_info_as_json_string(&pubkey.to_string()).unwrap();
    let expected = "{\"key\":\"HfaGtwhjsdjDvC4CmkHWJr91eT7wF3B1ipDFz5Lyy9YG\",\"owner\":\"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA\",\"data\":\"AAAAAAAAAAAAAAAAM1n1m25eyBbxI3fN88qSHBlp0mcAAAAAAAAAAAAAAABmIrn/z3lygrhqzvT2iK0a5daf89IEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",\"lamports\":\"2039280\"}";
    assert_eq!(expected, account_info);
}

#[test]
fn it_should_get_token_accounts_by_owner() {
    setup();
    create_token_account("f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    create_token_account("B2FeF42289bBdea2d1E3d712202b3Ca2C48eA989");
    let pubkey = "1111111111112RXi1yn6kTp7G8Td7o6z3Ciqw9v2"; // eq f39fd6e51aad88f6f4ce6ab8827279cfffb92266
    let accounts = inspect::get_token_accounts_by_owner(pubkey).unwrap();
    assert_eq!(accounts.len(), 1);
    let expected = "{\"key\":\"HfaGtwhjsdjDvC4CmkHWJr91eT7wF3B1ipDFz5Lyy9YG\",\"owner\":\"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA\",\"data\":\"AAAAAAAAAAAAAAAAM1n1m25eyBbxI3fN88qSHBlp0mcAAAAAAAAAAAAAAABmIrn/z3lygrhqzvT2iK0a5daf89IEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",\"lamports\":\"2039280\"}";
    assert_eq!(accounts[0], expected);
}

#[test]
fn it_should_get_token_accounts_by_owner_and_mint() {
    setup();
    create_token_account("f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
    create_token_account("B2FeF42289bBdea2d1E3d712202b3Ca2C48eA989");
    create_token_account_with_mint("f39fd6e51aad88f6f4ce6ab8827279cfffb92266", "f39fd6e51aad88f6f4ce6ab8827279cfffb92262");
    let mint = eth_address_to_pubkey(&hex::decode("67d269191c92caf3cd7723f116c85e6e9bf55933").unwrap());
    let url_path = format!("1111111111112RXi1yn6kTp7G8Td7o6z3Ciqw9v2?mint={:?}", mint);
    let accounts = inspect::handle_token_accounts_by_owner(&url_path).unwrap();
    assert_eq!(accounts.len(), 1);
    let expected = "{\"key\":\"HfaGtwhjsdjDvC4CmkHWJr91eT7wF3B1ipDFz5Lyy9YG\",\"owner\":\"TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA\",\"data\":\"AAAAAAAAAAAAAAAAM1n1m25eyBbxI3fN88qSHBlp0mcAAAAAAAAAAAAAAABmIrn/z3lygrhqzvT2iK0a5daf89IEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\",\"lamports\":\"2039280\"}";
    assert_eq!(accounts[0], expected);
}
