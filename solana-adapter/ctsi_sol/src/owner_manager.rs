use anchor_lang::prelude::Pubkey;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static mut OWNERS: Lazy<Vec<Pubkey>> = Lazy::new(|| vec![]);

pub static mut POINTERS: Mutex<Vec<(*mut &Pubkey, Pubkey)>> = Mutex::new(vec![]);

pub fn add_ptr(p: *mut Pubkey, key: Pubkey) {
    unsafe {
        POINTERS.lock().unwrap().push((p as *mut &Pubkey, key));
    }
}

pub fn change_owner<'a>(key: Pubkey, new_owner: Pubkey) {
    unsafe {
        let tot = OWNERS.len();
        OWNERS.push(new_owner);
        let pointers = &POINTERS.lock().unwrap();
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
