use anchor_lang::prelude::{AccountInfo, Pubkey};
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{collections::HashMap, str::FromStr};

pub static mut INDEX: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    let m: HashMap<String, usize> = HashMap::new();
    m
});

pub static mut OWNERS: Lazy<Vec<Pubkey>> = Lazy::new(|| vec![]);

pub static mut POINTERS: Lazy<Vec<(*mut &Pubkey, Pubkey)>> = Lazy::new(|| vec![]);

pub struct Manager<'a> {
    pub pointers: Vec<(*mut &'a Pubkey, &'a Pubkey)>,
    pub keys: Vec<Rc<&'a Pubkey>>,
    pub owners: Vec<&'a Pubkey>,
    pub accounts: Vec<AccountInfo<'a>>,
    pub lamports: Vec<Rc<RefCell<&'a mut u64>>>,
    pub info_data: Vec<Rc<RefCell<&'a mut [u8]>>>,
}

pub static mut MANAGER: Lazy<Arc<Mutex<Manager>>> =
    Lazy::new(|| Arc::new(Mutex::new(Manager::new())));

trait GetAccountInfo {
    fn get(&self, i: usize);
}

impl GetAccountInfo for Manager<'_> {
    fn get<'a>(&'a self, i: usize) {
        let mut account_info: AccountInfo = AccountInfo {
            key: &self.keys[i],
            is_signer: true,
            is_writable: true,
            lamports: Rc::clone(&self.lamports[i]),
            data: Rc::clone(&self.info_data[i]),
            owner: &self.owners[i],
            executable: false,
            rent_epoch: 0,
        };
    }
}

impl Manager<'_> {
    pub fn new() -> Self {
        Self {
            pointers: Vec::new(),
            keys: Vec::new(),
            accounts: Vec::new(),
            owners: Vec::new(),
            lamports: Vec::new(),
            info_data: Vec::new(),
        }
    }
    pub fn build_account_info(&mut self, key: String) {
        let pub_key = Pubkey::from_str(&key).unwrap();
        let rc = Rc::new(pub_key);
        let owner: Pubkey = Pubkey::default();
        let mut lamports: u64 = 1000;
        let mut info_data: Vec<u8> = Vec::new();
        // let mut account_info: AccountInfo = AccountInfo {
        //     key: &self.keys[0],
        //     is_signer: true,
        //     is_writable: true,
        //     lamports: Rc::new(RefCell::new(&mut lamports)),
        //     data: Rc::new(RefCell::new(&mut info_data)),
        //     owner: &owner,
        //     executable: false,
        //     rent_epoch: 0,
        // };
        // self.accounts.push(account_info);
    }
    pub fn change_the_owner<'a>(manager: &'a Manager<'a>, key: Pubkey, new_owner: &'a Pubkey) {
        let mut i = 0;
        for item in manager.pointers.iter() {
            if item.1.to_string() == key.to_string() {
                unsafe {
                    println!("OLD!!! {:?} {:?}", i, *item.0);
                    println!("ADDRESS!!! {:?}", item.0);
                    *item.0 = &new_owner;
                    println!("CHANGED!!! {:?}", &new_owner);
                }
            }
            i = i + 1;
        }
    }
}

pub fn put(key: String, owner: String) -> usize {
    unsafe {
        let contains = INDEX.contains_key(&key);
        match contains {
            true => 0,
            false => {
                let index = OWNERS.len();
                let pk = Pubkey::from_str(&owner).unwrap();
                OWNERS.push(pk);
                INDEX.insert(key.clone(), index);
                index
            }
        }
    }
}
