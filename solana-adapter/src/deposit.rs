use ctsi_sol::account_manager::{self, AccountFileData};
use ctsi_sol::adapter::eth_address_to_pubkey;
use ctsi_sol::anchor_lang::prelude::Rent;
use ctsi_sol::anchor_spl::token::{self};
use ctsi_sol::anchor_spl::NativeAccountData;
use ethabi::ethereum_types::U256;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use spl_token::state::{Account as InnerTokenAccount, AccountState as TokenAccountState};

struct Deposit {
    mint: Pubkey,
    amount: u64,
    owner: Pubkey,
}

pub fn create_token_account(payload: &str, _msg_sender: &str, _timestamp: &str) -> Pubkey {
    let bytes = hex::decode(&payload[2..]).unwrap();

    let deposit = decode_deposit(&bytes);
    let key = spl_associated_token_account::get_associated_token_address(&deposit.owner, &deposit.mint);
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
