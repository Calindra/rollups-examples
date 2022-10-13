use std::{cell::RefCell, rc::Rc, str::FromStr};

use anchor_lang::prelude::{AccountInfo, Pubkey};
use ctsi_sol::mem_acc::MemAccManager;
use ctsi_sol::mem_acc::OWNERS;
use once_cell::sync::Lazy;

#[test]
fn it_should_change_the_owner() {
    let owner = MemAccManager::put("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv".to_string(), "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv".to_string());
    let key: Pubkey = Pubkey::default();
    let mut lamports: u64 = 1000;
    let mut info_data: Vec<u8> = Vec::new();
    let account_info: AccountInfo = unsafe {
        AccountInfo {
            key: &key,
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            data: Rc::new(RefCell::new(&mut info_data)),
            owner: &OWNERS[owner],
            executable: false,
            rent_epoch: 0,
        }
    };
    assert_eq!(account_info.owner.to_string(), "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv");

    MemAccManager::change("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv".to_string(), "2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv".to_string());
    assert_eq!(account_info.owner.to_string(), "2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv");
}

#[test]
fn it_should_change_the_key() {
    static mut ids: Lazy<Vec<Pubkey>> =
        Lazy::new(|| vec![Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap()]);
    let owner: Pubkey = Pubkey::default();
    let mut lamports: u64 = 1000;
    let mut info_data: Vec<u8> = Vec::new();
    let account_info: AccountInfo = unsafe {
        AccountInfo {
            key: &ids[0],
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(&mut lamports)),
            data: Rc::new(RefCell::new(&mut info_data)),
            owner: &owner,
            executable: false,
            rent_epoch: 0,
        }
    };
    // MemAccManager::put(&id, &account_info);
    assert_eq!(
        account_info.key.to_string(),
        "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv"
    );
    unsafe {
        ids[0] = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    }

    assert_eq!(
        account_info.key.to_string(),
        "2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv"
    );
}
