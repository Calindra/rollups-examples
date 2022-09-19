use std::{str::FromStr, rc::Rc, cell::RefCell};

use anchor_lang::prelude::{Pubkey, AccountInfo, Account};
use ctsi_sol::AccountManager;
use solana_adapter::{call_smart_contract, self};
use solana_smart_contract::models::Zendao;

fn load_account_info_data(pubkey: &Pubkey) -> (Vec<u8>, u64, Pubkey) {
    let mut account_manager = AccountManager::new().unwrap();
    account_manager.set_base_path("tests/fixtures".to_owned());
    const MAX_SIZE: usize = 2000;
    let lamports = 1000;
    let read_account_data_file = account_manager.read_account(&pubkey);
    match read_account_data_file {
        Ok(account_data_file) => {
            return (
                account_data_file.data,
                account_data_file.lamports,
                account_data_file.owner,
            )
        }
        Err(_) => {
            let zeroes: [u8; MAX_SIZE] = [0; MAX_SIZE];
            let info_data = zeroes.to_vec();
            
            let owner = Pubkey::from_str("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv").unwrap();
            // let owner = Pubkey::default();
            return (info_data, lamports, owner);
        }
    };
}

#[test]
fn it_should_call_adapter() {
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));

    call_smart_contract(&hex_payload);
}

// #[test]
fn it_should_deserialize_transaction_from_base64_and_call_anchors_entry() {
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let decoded = base64::decode(encoded64).unwrap();
    let tx: solana_sdk::transaction::Transaction = bincode::deserialize(&decoded).unwrap();

    let program_id = solana_smart_contract::ID;
    let first = &tx.message.instructions[0];

    let mut accounts = Vec::new();
    let mut params = Vec::new();
    for pubkey in tx.message.account_keys.iter() {
        let (a, b, c) = load_account_info_data(&pubkey);
        params.push((a, b, c, pubkey));
    }
    for param in params.iter_mut() {
        accounts.push(AccountInfo {
            key: &param.3,
            is_signer: true,
            is_writable: true,
            lamports: Rc::new(RefCell::new(&mut param.1)),
            data: Rc::new(RefCell::new(&mut param.0)),
            owner: &param.2,
            executable: true,
            rent_epoch: 1,
        });
    }
    println!("accounts indexes {:?}", first.accounts);
    println!("method dispatch's sighash = {:?}", &first.data[..8]);
    let mut ordered_accounts = Vec::new();
    for index in first.accounts.iter() {
        let i: usize = (*index).into();
        ordered_accounts.push(accounts[i].to_owned());
    }
    for acc in ordered_accounts.iter() {
        println!("ordered_accounts = {:?}", acc.key);
    }
    solana_smart_contract::entry(&program_id, &ordered_accounts, &first.data).unwrap();

    let res = Account::<Zendao>::try_from_unchecked(&ordered_accounts[0]).unwrap();
    assert_eq!(
        res.token.to_string(),
        "CasshNb6PacBzSwbd5gw8uqoQEjcWxaQ9u9byFApShwT"
    );
    assert_eq!(res.min_balance, 1000);
    assert_eq!(res.slug, "slug");
}