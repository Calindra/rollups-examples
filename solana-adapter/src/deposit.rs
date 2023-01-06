use anchor_spl::token::TokenAccount;
use cartesi_solana::account_manager::{self, AccountFileData};
use cartesi_solana::adapter::eth_address_to_pubkey;
use ethabi::ethereum_types::U256;
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;
use std::cell::RefCell;
use std::rc::Rc;

use crate::token_account::{self, TokenAccountBasicData};

static mut ALLOWED_ADDRESS: Option<String> = None;

pub struct Deposit {
    mint: Pubkey,
    amount: u64,
    owner: Pubkey,
}

fn panic_check_msg_sender(msg_sender: &str) {
    unsafe {
        if msg_sender != ALLOWED_ADDRESS.as_ref().unwrap() {
            panic!("Address not allowed to make deposits");
        }
    }
}

pub fn only_accepts_deposits_from_address(address: String) {
    unsafe {
        ALLOWED_ADDRESS = Some(address);
    }
}

pub fn process(payload: &str, msg_sender: &str, timestamp: &str) -> Pubkey {
    panic_check_msg_sender(msg_sender);
    let bytes = hex::decode(&payload[2..]).unwrap();

    let deposit = decode_deposit(&bytes);
    let key =
        spl_associated_token_account::get_associated_token_address(&deposit.owner, &deposit.mint);
    let account_manager = account_manager::create_account_manager();
    let read = account_manager.read_account(&key);
    match read {
        Ok(account_file_data) => {
            add(&key, account_file_data, deposit);
        }
        Err(e) => {
            if let Some(error) = e.downcast_ref::<std::io::Error>() {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        create_token_account(payload, msg_sender, timestamp);
                    }
                    _ => {
                        panic!("Unexpected account error: {:?}", key)
                    }
                }
            }
        }
    }
    key
}

pub fn add(key: &Pubkey, account_info_data: AccountFileData, deposit: Deposit) {
    let mut lamports = account_info_data.lamports;
    let mut data = account_info_data.data;
    let account_info = AccountInfo {
        key,
        is_signer: false,
        is_writable: false,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut data)),
        owner: &account_info_data.owner,
        executable: false,
        rent_epoch: 0,
    };
    let token_acc: anchor_lang::accounts::account::Account<TokenAccount> =
        anchor_lang::accounts::account::Account::try_from_unchecked(&account_info).unwrap();
    let token_account_data = TokenAccountBasicData {
        mint: deposit.mint,
        amount: token_acc
            .amount
            .checked_add(deposit.amount)
            .expect("Token amount overflow u64"),
        owner: deposit.owner,
    };
    token_account::save_token_account(token_account_data, &key);
}

pub fn create_token_account(payload: &str, _msg_sender: &str, _timestamp: &str) -> Pubkey {
    let bytes = hex::decode(&payload[2..]).unwrap();
    let deposit = decode_deposit(&bytes);
    let key =
        spl_associated_token_account::get_associated_token_address(&deposit.owner, &deposit.mint);
    let token_account_data = TokenAccountBasicData {
        mint: deposit.mint,
        amount: deposit.amount,
        owner: deposit.owner,
    };
    token_account::save_token_account(token_account_data, &key);
    key
}

fn decode_deposit(bytes: &[u8]) -> Deposit {
    let header = &bytes[0..32];

    // owner address
    let depositor = &bytes[(32 + 12)..(32 + 32)];

    // token/coin
    let erc20 = &bytes[(32 + 32 + 12)..(32 + 32 + 32)];

    let amount: U256 = (&bytes[(32 + 32 + 32)..(32 + 32 + 32 + 32)]).into();
    println!("header = {}", hex::encode(header));
    println!("depositor = {}", hex::encode(depositor));
    println!("erc20 = {}", hex::encode(erc20));
    println!("amount = {}", amount.as_u64());

    let mint = eth_address_to_pubkey(erc20);
    let owner = eth_address_to_pubkey(depositor);
    Deposit {
        mint,
        owner,
        amount: amount.as_u64(),
    }
}
