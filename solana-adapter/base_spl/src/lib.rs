use anchor_lang::{
    prelude::{AccountInfo, Pubkey},
    solana_program::msg,
};
use cartesi_solana::{account_manager, owner_manager};
use serde::{Deserialize, Serialize};

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
    msg!("Base SPL Instruction.code = {}", instruction.code);
    if instruction.code == 0 {
        let create: Create = bincode::deserialize(data).unwrap();
        let from = &accounts[0];
        let account = &accounts[1];
        transfer_lamports(from, account, create.lamports)?;
        account_manager::set_data_size(account, create.space.try_into().unwrap());
        println!(
            "create account {:?} with owner {:?}",
            account.key, create.program_id
        );
        owner_manager::change_owner(account.key.clone(), create.program_id);
    } else if instruction.code == 1 {
        let assing: Assing =
            bincode::deserialize(data).expect("Deserialize Assing instruction error");
        let account = &accounts[0];
        owner_manager::change_owner(account.key.clone(), assing.program_id);
    } else if instruction.code == 2 {
        let transfer: Transfer = bincode::deserialize(data).unwrap();
        msg!(
            "transfer lamports {} from {:?} to {:?}",
            transfer.lamports,
            accounts[0].key,
            accounts[1].key
        );
        let from = &accounts[0];
        let to = &accounts[1];
        if !from.is_signer && from.key.to_string() != AIRDROP_PUBKEY {
            panic!("Not signed transfer");
        }
        transfer_lamports(from, to, transfer.lamports)?;
    } else if instruction.code == 8 {
        let allocate: Allocate =
            bincode::deserialize(data).expect("Deserialize Allocate instruction error");
        let account = &accounts[0];
        account_manager::set_data_size(account, allocate.space.try_into().unwrap());
    } else {
        panic!("Instruction code {} not implemented", instruction.code);
    }
    Ok(())
}

fn transfer_lamports(
    from: &AccountInfo,
    to: &AccountInfo,
    lamports: u64,
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    **from.try_borrow_mut_lamports()? = from
        .lamports()
        .checked_sub(lamports)
        .expect("Underflow error");
    **to.try_borrow_mut_lamports()? = to.lamports().checked_add(lamports).expect("Overflow error");
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

#[derive(Serialize, Deserialize)]
pub struct Allocate {
    instruction: u32,
    space: u64,
}

#[derive(Serialize, Deserialize)]

pub struct Assing {
    instruction: u32,
    program_id: Pubkey,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Underflow error")]
    fn it_should_check_underflow() {
        let mut data = vec![];
        let mut data_to = vec![];
        let key = Pubkey::default();
        let owner = Pubkey::default();
        let mut lamports_from = 0;
        let mut lamports_to = 0;

        let from = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports_from,
            &mut data,
            &owner,
            false,
            0,
        );
        let to = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports_to,
            &mut data_to,
            &owner,
            false,
            0,
        );
        transfer_lamports(&from, &to, 1).unwrap();
    }

    #[test]
    #[should_panic(expected = "Overflow error")]
    fn it_should_check_overflow() {
        let mut data = vec![];
        let mut data_to = vec![];
        let key = Pubkey::default();
        let owner = Pubkey::default();
        let mut lamports_from = 1;
        let mut lamports_to = u64::MAX;

        let from = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports_from,
            &mut data,
            &owner,
            false,
            0,
        );
        let to = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports_to,
            &mut data_to,
            &owner,
            false,
            0,
        );
        transfer_lamports(&from, &to, 1).unwrap();
    }

    #[test]
    fn it_should_transfer() {
        let mut data = vec![];
        let mut data_to = vec![];
        let key = Pubkey::default();
        let owner = Pubkey::default();
        let mut lamports_from = 7;
        let mut lamports_to = 3;

        let from = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports_from,
            &mut data,
            &owner,
            false,
            0,
        );
        let to = AccountInfo::new(
            &key,
            true,
            true,
            &mut lamports_to,
            &mut data_to,
            &owner,
            false,
            0,
        );
        transfer_lamports(&from, &to, 5).unwrap();
        assert_eq!(lamports_from, 2);
        assert_eq!(lamports_to, 8);
    }
}
