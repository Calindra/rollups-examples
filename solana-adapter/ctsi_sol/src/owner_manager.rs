use std::fs;
use ::anchor_lang::prelude::Pubkey;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind::NotFound;
pub static mut OWNERS: Lazy<Vec<Pubkey>> = Lazy::new(|| vec![]);

/*
#0 170.8 error[E0015]: cannot call non-const fn `Mutex::<Vec<(*mut &Pubkey, Pubkey)>>::new` in statics
#0 170.8  --> ctsi_sol/src/owner_manager.rs:7:63
#0 170.8   |
#0 170.8 7 | pub static mut POINTERS: Mutex<Vec<(*mut &Pubkey, Pubkey)>> = Mutex::new(vec![]);
#0 170.8   |                                                               ^^^^^^^^^^^^^^^^^^
#0 170.8   |
#0 170.8   = note: calls in statics are limited to constant functions, tuple structs and tuple variants
*/
pub static mut POINTERS: Lazy<Vec<(*mut &Pubkey, Pubkey)>> = Lazy::new(|| vec![]);

pub fn add_ptr(p: *mut Pubkey, key: Pubkey) {
    unsafe {
        POINTERS.push((p as *mut &Pubkey, key));
    }
}

pub fn change_owner<'a>(key: Pubkey, new_owner: Pubkey) {
    unsafe {
        let tot = OWNERS.len();
        OWNERS.push(new_owner);
        let pointers = &POINTERS;
        let mut i = 0;
        for item in pointers.iter() {
            if item.1.to_string() == key.to_string() {
                let old = *item.0;
                *item.0 = &OWNERS[tot];
                anchor_lang::prelude::msg!(
                    "change_owner: i[{}] account[{:?}] old[{:?}] new[{:?}]",
                    i,
                    key,
                    old,
                    new_owner
                );
            }
            i = i + 1;
        }
    }
}


pub struct AccountManager {
    base_path: String,
}

impl AccountManager {
    pub fn new() -> std::result::Result<AccountManager, Box<dyn std::error::Error>> {
        Ok(Self {
            base_path: "tests/fixtures".to_string(),
        })
    }

    pub fn set_base_path(&mut self, base_path: String) {
        self.base_path = base_path;
    }

    pub fn write_account(
        &self,
        pubkey: &Pubkey,
        account_file_data: &AccountFileData,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}.json", &self.base_path, pubkey.to_string());
        let contents = serde_json::to_string(account_file_data)?;
        fs::write(file_path, contents)?;
        Ok(())
    }

    pub fn read_account(
        &self,
        pubkey: &Pubkey,
    ) -> std::result::Result<AccountFileData, Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}.json", &self.base_path, pubkey.to_string());
        let contents = fs::read_to_string(file_path)?;
        let account = serde_json::from_str::<AccountFileData>(&contents)?;
        Ok(account)
    }

    pub fn delete_account(
        &self,
        pubkey: &Pubkey,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}.json", &self.base_path, pubkey.to_string());
        let delete_result = fs::remove_file(file_path);
        match delete_result {
            Ok(_) => {
                return Ok(());
            },
            Err(error) => {
                if error.kind() == NotFound {
                    return Ok(())
                } else {
                    return Err(Box::new(error))
                }
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AccountFileData {
    pub owner: Pubkey,
    pub data: Vec<u8>,
    pub lamports: u64,
}