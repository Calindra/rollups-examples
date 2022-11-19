use std::cell::RefCell;
use std::rc::Rc;

use ctsi_sol::account_manager::{self, AccountFileData};
use ctsi_sol::adapter::eth_address_to_pubkey;
use ctsi_sol::anchor_lang;
use ctsi_sol::anchor_lang::prelude::Rent;
use ctsi_sol::anchor_spl::token::{self, TokenAccount};
use ctsi_sol::anchor_spl::NativeAccountData;
use ethabi::ethereum_types::U256;
use solana_program::account_info::AccountInfo;
use solana_program::program_pack::Pack;
use solana_program::pubkey::{Pubkey, self};
use spl_token::state::{Account as InnerTokenAccount, AccountState as TokenAccountState};

pub struct Deposit {
    mint: Pubkey,
    amount: u64,
    owner: Pubkey,
}

pub fn process(payload: &str, msg_sender: &str, timestamp: &str) -> Pubkey {
    let bytes = hex::decode(&payload[2..]).unwrap();

    let deposit = decode_deposit(&bytes);
    let key = spl_associated_token_account::get_associated_token_address(&deposit.owner, &deposit.mint);
    let account_manager = account_manager::create_account_manager();
    let read = account_manager.read_account(&key);
    match read {
        Ok(account_file_data) => {
            add(&key, account_file_data, deposit);
        },
        Err(error) => {
            create_token_account(payload, msg_sender, timestamp);
        },
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
    let token_account: anchor_lang::accounts::account::Account<TokenAccount> =
        anchor_lang::accounts::account::Account::try_from_unchecked(&account_info).unwrap();
    let mut deposit = deposit;
    deposit.amount  = token_account.amount + deposit.amount;
    save_token_account(deposit, &key);
}



pub fn create_token_account(payload: &str, _msg_sender: &str, _timestamp: &str) -> Pubkey {
    let bytes = hex::decode(&payload[2..]).unwrap();

    let deposit = decode_deposit(&bytes);
    let key = spl_associated_token_account::get_associated_token_address(&deposit.owner, &deposit.mint);
    save_token_account(deposit, &key);
    key
}

fn save_token_account(deposit: Deposit, key: &Pubkey) {
    let token_account = InnerTokenAccount {
        mint: deposit.mint,
        owner: deposit.owner,
        amount: deposit.amount,
        state: TokenAccountState::Initialized,
        ..Default::default()
    };
    let mut account_data = NativeAccountData::new(InnerTokenAccount::LEN, spl_token::id());
    InnerTokenAccount::pack(token_account, &mut account_data.data).unwrap();
    let lamports = Rent::get().unwrap().minimum_balance(InnerTokenAccount::LEN);
    let account_file_data = AccountFileData {
        owner: token::ID,
        data: account_data.data.to_vec(),
        lamports,
    };
    let account_manager = account_manager::create_account_manager();
    account_manager
        .write_account(&key, &account_file_data)
        .unwrap();
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
