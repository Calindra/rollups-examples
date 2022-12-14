use cartesi_solana::{anchor_lang::{self, prelude::{AccountInfo, Pubkey}, solana_program::msg}, account_manager};
use serde::{Serialize, Deserialize};

const AIRDROP_PUBKEY: &str = "9B5XszUGdMaxCZ7uSQhPzdks5ZQSmWxrmzCSvtJ6Ns6g";

pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    if program_id.to_string() != "11111111111111111111111111111111" {
        panic!("Wrong program id");
    }
    let instruction: Instruction = bincode::deserialize(&data[..4]).unwrap();
    msg!("instruction = {}", instruction.code);
    if instruction.code == 0 {
        let create: Create = bincode::deserialize(data).unwrap();
        let from = &accounts[0];
        let account = &accounts[1];
        **from.try_borrow_mut_lamports()? -= create.lamports;
        **account.try_borrow_mut_lamports()? += create.lamports;
        account_manager::set_data_size(account, create.space.try_into().unwrap());
    } else if instruction.code == 2 {
        let transfer: Transfer = bincode::deserialize(data).unwrap();
        msg!("transfer lamports {} from {:?} to {:?}", transfer.lamports, accounts[0].key, accounts[1].key);
        let from = &accounts[0];
        let to = &accounts[1];
        if !from.is_signer && from.key.to_string() != AIRDROP_PUBKEY {
            panic!("Not signed transfer");
        }
        **from.try_borrow_mut_lamports()? -= transfer.lamports;
        **to.try_borrow_mut_lamports()? += transfer.lamports;
    } else {
        msg!("Instruction code {} not implemented", instruction.code);
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct Instruction {
    code: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Transfer {
    instruction: u32,
    lamports: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Create {
    instruction: u32,
    lamports: u64,
    space: u64,
    program_id: Pubkey,
}
