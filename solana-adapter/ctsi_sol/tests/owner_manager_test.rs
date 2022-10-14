use std::sync::Arc;
use std::{cell::RefCell, ptr, rc::Rc, str::FromStr};

use anchor_lang::prelude::{AccountInfo, Pubkey};
use ctsi_sol::owner_manager::MANAGER;
use ctsi_sol::owner_manager::OWNERS;
use ctsi_sol::owner_manager::{self, Manager};

use once_cell::sync::Lazy;

fn entry(accounts: &[AccountInfo]) {
    println!(" owner == {:?}", accounts[0].owner);
}

fn entry_owner(accounts: Vec<AccountInfo>) {
    println!(" owner == {:?}", accounts[0].owner);
}

fn entry_mut(accounts: &mut [AccountInfo]) {
    println!(" owner == {:?}", accounts[0].owner);
}

fn entry2(accounts: &[&Box<AccountInfo>]) {
    println!(" 2 owner == {:?}", accounts[0].owner);
}

#[test]
fn it_should_pointer_3_cont() {
    let owner: Pubkey = Pubkey::default();
    let key = &Pubkey::default();
    let mut lamports: u64 = 1000;
    let mut info_data: Vec<u8> = Vec::new();
    let mut account_info: AccountInfo = AccountInfo {
        key,
        is_signer: true,
        is_writable: true,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut info_data)),
        owner: &owner,
        executable: false,
        rent_epoch: 0,
    };
    unsafe {
        let arc = Arc::clone(&MANAGER);
        let mut manager = arc.lock().unwrap();
        let p = std::ptr::addr_of_mut!(account_info.owner);
        let new_owner = Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap();
        // manager.pointers.push((p, &new_owner));
        manager.build_account_info(key.to_string());
    }
}

#[test]
fn it_should_learn_about_box() {
    let owner: Pubkey = Pubkey::default();
    let key = &Pubkey::default();
    let mut lamports: u64 = 1000;
    let mut info_data: Vec<u8> = Vec::new();
    let account_info: AccountInfo = AccountInfo {
        key,
        is_signer: true,
        is_writable: true,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut info_data)),
        owner: &owner,
        executable: false,
        rent_epoch: 0,
    };
    let mut new_acc = account_info.clone();
    let mut new_acc3 = account_info.clone();
    let new_owner = Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap();
    new_acc.owner = &new_owner;
    let mut b = Box::new(account_info);
    println!("b = {:?}", b);

    let accounts: &[AccountInfo] = &[*b];

    *b = new_acc;
    entry(&accounts);

    println!("owner = {:?}", b.owner);

    let accounts2: &[&Box<AccountInfo>] = &[&b];
    entry2(accounts2);

    // *b = new_acc3;
    // entry2(accounts2);
}

#[test]
fn it_should_change_the_owner_pointer_1() {
    let my_num: i32 = 10;
    let _my_num_ptr: *const i32 = &my_num;
    let mut my_speed: i32 = 88;
    {
        let my_speed_ptr: *mut i32 = &mut my_speed;
        let speed_ptr = unsafe { my_speed_ptr.as_mut().unwrap() };
        *speed_ptr = 55;
    }
    println!("\n {:?}", my_speed);
}

#[test]
fn it_should_change_the_owner_pointer_4() {
    let mut accounts = Vec::new();
    let owner: Pubkey = Pubkey::default();
    let key = &Pubkey::default();
    let mut lamports: u64 = 1000;
    let mut info_data: Vec<u8> = Vec::new();
    let account_info: AccountInfo = AccountInfo {
        key, // path ou endereco da conta
        owner: &owner,
        is_signer: true,
        is_writable: true,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut info_data)),
        executable: false,
        rent_epoch: 0,
    };
    let mut manager = Manager::new();
    
    let account_info2 = account_info.clone();

    accounts.push(account_info);
    let p = std::ptr::addr_of_mut!(accounts[0].owner);
    manager.pointers.push((p, key));

    accounts.push(account_info2);
    // let manager = fun_name(&accounts, key);
    // entry_owner(accounts);

    let new_owner = Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap();
    Manager::change_the_owner(&manager, key.clone(), &new_owner);

    assert_eq!(
        accounts[0].owner.to_string(),
        "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv"
    );

    // let new_owner2 = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    // Manager::change_the_owner(&manager, key.clone(), &new_owner2);
    entry(&accounts);
    // assert_eq!(
    //     accounts[0].owner.to_string(),
    //     "2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv"
    // );
    // free manager and account
}

fn fun_name<'a>(accounts: &'a Vec<AccountInfo<'a>>, key: &'a Pubkey) -> Manager<'a> {
    let mut pointers = Vec::new();
    let tot = accounts.len();
    for i in 0..tot {
        // Implicit coercion:
        // let mut m: u32 = 2;
        // let p: *mut &Pubkey = &mut accounts[i].owner;

        // // let p = std::ptr::addr_of_mut!(accounts[i].owner);
        // pointers.push((p, key));
    }
    let mut manager = Manager::new();
    manager.pointers = pointers;
    manager
}

#[test]
fn it_should_change_the_owner_pointer_3() {
    let mut accounts = Vec::new();
    let owner: Pubkey = Pubkey::default();
    let key = &Pubkey::default();
    let mut lamports: u64 = 1000;
    let mut info_data: Vec<u8> = Vec::new();
    let account_info: AccountInfo = AccountInfo {
        key, // path ou endereco da conta
        is_signer: true,
        is_writable: true,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut info_data)),
        owner: &owner,
        executable: false,
        rent_epoch: 0,
    };
    let account_info2 = account_info.clone();
    accounts.push(account_info);
    accounts.push(account_info2);
    let mut pointers = Vec::new();
    let tot = accounts.len();
    for i in 0..tot {
        let p: *mut &Pubkey = std::ptr::addr_of_mut!(accounts[i].owner);
        pointers.push((p, key));
    }

    let mut manager = Manager::new();
    manager.pointers = pointers;

    let new_owner = Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap();
    Manager::change_the_owner(&manager, key.clone(), &new_owner);

    assert_eq!(
        accounts[0].owner.to_string(),
        "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv"
    );

    entry(&accounts);

    assert_eq!(
        accounts[0].owner.to_string(),
        "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv"
    );

    let new_owner2 = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
    Manager::change_the_owner(&manager, key.clone(), &new_owner2);
    entry(&accounts);
    assert_eq!(
        accounts[0].owner.to_string(),
        "2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv"
    );
}

#[test]
fn it_should_change_the_owner_pointer_2() {
    let owner: Pubkey = Pubkey::default();
    let key = &Pubkey::default();
    let mut lamports: u64 = 1000;
    let mut info_data: Vec<u8> = Vec::new();
    let account_info: AccountInfo = AccountInfo {
        key,
        is_signer: true,
        is_writable: true,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut info_data)),
        owner: &owner,
        executable: false,
        rent_epoch: 0,
    };
    let new_owner = Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap();
    let mut account_info2 = account_info.clone();
    account_info2.owner = &new_owner;

    // change_owner(&account_info, &account_info2);
    // assert_eq!(account_info.owner.to_string(), "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv");
    let mut accounts = vec![account_info];
    let pointers = accounts.as_mut_ptr();
    unsafe {
        *pointers = account_info2;
    }
    assert_eq!(
        accounts[0].owner.to_string(),
        "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv"
    );

    // let new_owner = Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap();
    // unsafe {
    //     (*pointers).owner = &new_owner;
    // }
    // let ptr = account_info.owner.as_mut_ptr();
    // {
    //     let my_owner_ptr: *mut Pubkey = &mut owner;
    //     let owner_ptr = unsafe { my_owner_ptr.as_mut().unwrap() };
    //     *owner_ptr = Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap();
    // }

    // ok
    // unsafe {
    //     ptr::replace(&mut account_info.owner, &new_owner);
    // }

    // change_owner(&mut owner, new_owner);

    // println!("\n{:?}", &owner.to_string());
    // account_info
    // assert_eq!(account_info.owner.to_string(), "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv");

    // assert_eq!(&accounts[0].owner.to_string(), "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv");
}

// #[test]
// fn it_should_change_the_owner_fail() {
//     // This doesnt works
//     let owner = owner_manager::put(
//         "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv".to_string(),
//         "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv".to_string(),
//     );
//     let key: Pubkey = Pubkey::default();
//     let mut lamports: u64 = 1000;
//     let mut info_data: Vec<u8> = Vec::new();
//     let account_info: AccountInfo = unsafe {
//         AccountInfo {
//             key: &key,
//             is_signer: true,
//             is_writable: true,
//             lamports: Rc::new(RefCell::new(&mut lamports)),
//             data: Rc::new(RefCell::new(&mut info_data)),
//             owner: &OWNERS[owner],
//             executable: false,
//             rent_epoch: 0,
//         }
//     };
//     assert_eq!(
//         account_info.owner.to_string(),
//         "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv"
//     );

//     owner_manager::change(
//         "EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv".to_string(),
//         "2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv".to_string(),
//     );
//     assert_eq!(
//         account_info.owner.to_string(),
//         "2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv"
//     );
// }

#[test]
fn it_should_change_the_key() {
    static mut ids: Lazy<Vec<Pubkey>> = Lazy::new(|| {
        vec![Pubkey::from_str("EwiqbApgaLT2kQaohqZnSXT9HbkMQWDektXEjXGMJyJv").unwrap()]
    });
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
