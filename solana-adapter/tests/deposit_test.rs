use std::{
    cell::RefCell,
    fs,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use ctsi_sol::{
    account_manager::create_account_manager, adapter::eth_address_to_pubkey, anchor_lang,
    anchor_spl::token::TokenAccount, owner_manager,
};
use solana_adapter::deposit::{self};
use solana_program::account_info::AccountInfo;

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
    std::env::set_var("PORTAL_ADDRESS", "0xf8c694fd58360de278d5ff2276b7130bfdc0192a");
    unsafe {
        owner_manager::POINTERS.clear();
        owner_manager::OWNERS.clear();
    }
}

#[test]

fn it_should_create_a_token_account() {
    setup();
    let payload = "0x59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000067d269191c92caf3cd7723f116c85e6e9bf5593300000000000000000000000000000000000000000000000000000000000004d200000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
    let msg_sender = "0xf8c694fd58360de278d5ff2276b7130bfdc0192a";
    let timestamp = "1";
    let pubkey = deposit::create_token_account(payload, msg_sender, timestamp);
    let account_manager = create_account_manager();
    let account_info_data = account_manager.read_account(&pubkey).unwrap();
    let mut lamports = account_info_data.lamports;
    let mut data = account_info_data.data;
    let account_info = AccountInfo {
        key: &pubkey,
        is_signer: false,
        is_writable: false,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut data)),
        owner: &account_info_data.owner,
        executable: false,
        rent_epoch: 0,
    };
    let token_account: anchor_lang::accounts::account::Account<TokenAccount> =
        anchor_lang::accounts::account::Account::try_from_unchecked(&account_info).unwrap();

    // depositor is the ethereum address 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
    // converted to a 32 bytes solana public key
    let depositor = "1111111111112RXi1yn6kTp7G8Td7o6z3Ciqw9v2";
    let eth_address = hex::decode("67d269191c92caf3cd7723f116c85e6e9bf55933").unwrap();
    let mint = eth_address_to_pubkey(&eth_address);
    assert_eq!(token_account.owner.to_string(), depositor);
    assert_eq!(token_account.amount, 1234);
    assert_eq!(token_account.mint, mint);
}

#[test]
fn it_should_create_and_add_to_a_token_account() {
    setup();
    let payload = "0x59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000067d269191c92caf3cd7723f116c85e6e9bf5593300000000000000000000000000000000000000000000000000000000000004d200000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
    let msg_sender = "0xf8c694fd58360de278d5ff2276b7130bfdc0192a";
    let timestamp = "1";
    let pubkey = deposit::process(payload, msg_sender, timestamp);

    // second deposit
    deposit::process(payload, msg_sender, timestamp);
    
    let account_manager = create_account_manager();
    let account_info_data = account_manager.read_account(&pubkey).unwrap();
    let mut lamports = account_info_data.lamports;
    let mut data = account_info_data.data;
    let account_info = AccountInfo {
        key: &pubkey,
        is_signer: false,
        is_writable: false,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut data)),
        owner: &account_info_data.owner,
        executable: false,
        rent_epoch: 0,
    };
    let token_account: anchor_lang::accounts::account::Account<TokenAccount> =
        anchor_lang::accounts::account::Account::try_from_unchecked(&account_info).unwrap();

    // depositor is the ethereum address 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
    // converted to a 32 bytes solana public key
    let depositor = "1111111111112RXi1yn6kTp7G8Td7o6z3Ciqw9v2";
    let eth_address = hex::decode("67d269191c92caf3cd7723f116c85e6e9bf55933").unwrap();
    let mint = eth_address_to_pubkey(&eth_address);
    assert_eq!(token_account.owner.to_string(), depositor);
    assert_eq!(token_account.amount, 1234*2);
    assert_eq!(token_account.mint, mint);
}