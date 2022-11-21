use std::{cell::RefCell, rc::Rc, error::Error};

use ctsi_sol::{
    account_manager::{self, AccountFileData},
    anchor_lang::{prelude::Rent, self},
    anchor_spl::{token::{self, TokenAccount}, NativeAccountData},
};
use solana_program::{program_pack::Pack, pubkey::Pubkey, account_info::AccountInfo};
use spl_token::state::{Account as InnerTokenAccount, AccountState as TokenAccountState};

pub struct TokenAccountBasicData {
    pub mint: Pubkey,
    pub amount: u64,
    pub owner: Pubkey,
}

pub fn subtract(token_account_address: &Pubkey, amount: &u64, check_owner: &Pubkey) -> Result<(), Box<dyn Error>> {
    let account_manager = account_manager::create_account_manager();
    let account_info_data = account_manager.read_account(token_account_address).unwrap();
    let mut lamports = account_info_data.lamports;
    let mut data = account_info_data.data;
    let account_info = AccountInfo {
        key: token_account_address,
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
    
    if token_acc.owner != *check_owner {
        panic!("Wrong token account owner");
    }
    let token_account_data = TokenAccountBasicData {
        mint: token_acc.mint,
        amount: token_acc.amount.checked_sub(*amount).unwrap(),
        owner: token_acc.owner,
    };
    save_token_account(token_account_data, &token_account_address);
    Ok(())
}

pub fn save_token_account(token_account_data: TokenAccountBasicData, key: &Pubkey) {
    let token_account = InnerTokenAccount {
        mint: token_account_data.mint,
        owner: token_account_data.owner,
        amount: token_account_data.amount,
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
