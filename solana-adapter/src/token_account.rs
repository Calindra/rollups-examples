use solana_program::{pubkey::Pubkey, program_pack::Pack};
use spl_token::state::{Account as InnerTokenAccount, AccountState as TokenAccountState};
use ctsi_sol::{anchor_spl::{NativeAccountData, token}, account_manager::{AccountFileData, self}, anchor_lang::prelude::Rent};


pub struct TokenAccountBasicData {
    pub mint: Pubkey,
    pub amount: u64,
    pub owner: Pubkey,
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