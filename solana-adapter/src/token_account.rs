use std::{cell::RefCell, error::Error, rc::Rc};

use anchor_lang::{self, prelude::Rent};
use anchor_spl::token::{self, TokenAccount};
use cartesi_solana::account_manager::{self, AccountFileData};
use solana_program::{
    account_info::AccountInfo, program_pack::Pack, pubkey::Pubkey, stake_history::Epoch,
};
use spl_token::state::{Account as InnerTokenAccount, AccountState as TokenAccountState};

pub struct TokenAccountBasicData {
    pub mint: Pubkey,
    pub amount: u64,
    pub owner: Pubkey,
}

pub fn subtract(
    token_account_address: &Pubkey,
    amount: &u64,
    check_owner: &Pubkey,
) -> Result<(), Box<dyn Error>> {
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
        amount: token_acc
            .amount
            .checked_sub(*amount)
            .expect("Insufficient funds"),
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
    let lamports = Rent::default().minimum_balance(InnerTokenAccount::LEN);
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

#[derive(Clone)]
pub struct NativeAccountData {
    pub key: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub program_id: Pubkey,
    pub is_signer: bool,
}

impl NativeAccountData {
    pub fn new(size: usize, program_id: Pubkey) -> Self {
        Self {
            key: Pubkey::new_unique(),
            lamports: 0,
            data: vec![0; size],
            program_id,
            is_signer: false,
        }
    }

    pub fn new_from_account_info(account_info: &AccountInfo) -> Self {
        Self {
            key: *account_info.key,
            lamports: **account_info.lamports.borrow(),
            data: account_info.data.borrow().to_vec(),
            program_id: *account_info.owner,
            is_signer: account_info.is_signer,
        }
    }

    pub fn as_account_info(&mut self) -> AccountInfo {
        AccountInfo::new(
            &self.key,
            self.is_signer,
            false,
            &mut self.lamports,
            &mut self.data[..],
            &self.program_id,
            false,
            Epoch::default(),
        )
    }
}
