use std::{cell::{RefCell, Ref}, rc::Rc, vec};

use ctsi_sol::anchor_spl::token;
use ethabi::ethereum_types::U256;
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

pub fn create_token_account(payload: &str, msg_sender: &str, timestamp: &str) {
    let bytes = hex::decode(&payload[2..]).unwrap();
    let header = &bytes[0..32];

    // portal address
    let depositor = &bytes[(32 + 12)..(32 + 32)];

    // token/coin
    let erc20 = &bytes[(32 + 32 + 12)..(32 + 32 + 32)];
    println!("header = {}", hex::encode(header));
    println!("depositor = {}", hex::encode(depositor));
    println!("erc20 = {}", hex::encode(erc20));
    let amount: U256 = (&bytes[(32 + 32 + 32)..(32 + 32 + 32 + 32)]).into();
    println!("amount = {}", amount);

    let key = &Pubkey::default();
    let mut lamports = 100000;
    let mut data = vec![];
    let account_info = AccountInfo {
        key,
        is_signer: false,
        is_writable: true,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut data)),
        owner: &token::ID,
        executable: false,
        rent_epoch: 0,
    };
}
