use anchor_lang::prelude::Pubkey;
use once_cell::sync::Lazy;
use std::{collections::HashMap, str::FromStr};

static mut INDEX: Lazy<HashMap<String, usize>> = Lazy::new(|| {
    let m: HashMap<String, usize> = HashMap::new();
    m
});

pub static mut OWNERS: Lazy<Vec<Pubkey>> =
        Lazy::new(|| vec![]);

pub struct MemAccManager {}

impl MemAccManager {
    pub fn change(key: String, owner: String) {
        unsafe {
            let item = INDEX.get(&key);
            match item {
                Some(i) => {
                    println!("index = {:?}", i);
                    OWNERS[*i] = Pubkey::from_str(&owner).unwrap();
                },
                None => {

                },
            }
        }
    }
    pub fn put(key: String, owner: String) -> usize {
        unsafe {
            let contains = INDEX.contains_key(&key);
            match  contains {
                true => {
                    0
                },
                false => {
                    let index = OWNERS.len();
                    let pk = Pubkey::from_str(&owner).unwrap();
                    OWNERS.push(pk);
                    INDEX.insert(key,index);
                    index
                },
            }
        }
    }
}
