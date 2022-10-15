use ctsi_sol::{anchor_lang::prelude::Pubkey, owner_manager};
use solana_adapter::{self, call_smart_contract, read_account_info_as_json};
use std::fs;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

fn setup() {
    println!("\n\n***** setup *****\n");
    let dir = std::env::temp_dir();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let final_temp_dir = format!(
        "{}/{}",
        dir.as_os_str().to_str().unwrap(),
        since_the_epoch.subsec_nanos()
    );
    println!("{}", final_temp_dir);
    fs::create_dir(&final_temp_dir).unwrap();
    std::env::set_var("SOLANA_DATA_PATH", final_temp_dir);
    unsafe {
        owner_manager::POINTERS.lock().unwrap().clear();
        owner_manager::OWNERS.clear();
    }
}

fn create_default_account() {
    let encoded64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAIFAAAAAAAAAAAAAAAAZiK5/895coK4as709oitGuXWn/M9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW79hOdgffqSFwR4MaqpqSzXlBSS0M5PrBq8FFzX/hnMW5AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUd2Jd/lKEoTVTiIYaxgO2ii/8ZHEybZ1ho7bLWzqpLo3AQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));
    let msg_sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    call_smart_contract(&hex_payload, &msg_sender);
}

#[test]
fn it_should_create_token_account() {
    setup();
    let encoded64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAUHAAAAAAAAAAAAAAAAZiK5/895coK4as709oitGuXWn/NRMLOxTZKDRoTgckYGMm7zeuFm6Fu9cFsRR2oRHd3NhwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFMuB2+5klhUE22BCHIc0cmVUS5eCi0BLmohvoVZCr1E6xifHi/mz/JHBouMDsoFNr79q7lwcL00uX4UZJehrvAan1RcZLFxRIYzJTD1K8X9Y2u4Im6H9ROPb2YoAAAAABt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKlxT484t040ZNvBoIHu+yWQcu7cGELHigZMqXbq2/DbkAEDBgEABAYCBQiNhOmCqLcKdw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));
    let msg_sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    call_smart_contract(&hex_payload, &msg_sender);
}

#[test]
fn it_should_call_adapter_without_errors_to_create_an_account() {
    setup();
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));
    let pubkey = Pubkey::from_str("83kEotjF8mm7wZPqqkTRGkmixoKQzzoeEaipxkSrQTjn").unwrap();

    // convert pubkey to ethereum address
    let tmp: Vec<u8> = pubkey.to_bytes()[12..].to_vec().into_iter().rev().collect();
    let sender_key = hex::encode(&tmp);
    let msg_sender = format!("0x{}", sender_key);

    println!("msg_sender = {}", msg_sender);
    call_smart_contract(&hex_payload, &msg_sender);
}

#[test]
fn it_should_call_adapter_to_update_an_account() {
    setup();
    create_default_account();
    let encoded64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAIEAAAAAAAAAAAAAAAAZiK5/895coK4as709oitGuXWn/M9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW7wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFMuB2+5klhUE22BCHIc0cmVUS5eCi0BLmohvoVZCr1HGO2lyHc2PgnUrXxhtRC24Y6nYeOv6RF4eCOAX/5NcuwEDAwEAAjDbyFiwnj/9fwabiFf+q4GE+2h/Y0YYwDXaxDncGus7VZig8AAAAAAB0gQAAAAAAAA=";
    let hex_payload = format!("0x{}", hex::encode(encoded64));
    let msg_sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    call_smart_contract(&hex_payload, &msg_sender);
}

#[test]
fn it_should_call_adapter_to_close_an_account() {
    setup();
    create_default_account();
    let encoded64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAIEAAAAAAAAAAAAAAAAZiK5/895coK4as709oitGuXWn/M9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW7wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFMuB2+5klhUE22BCHIc0cmVUS5eCi0BLmohvoVZCr1E/jzRRsjOOHn3eqsuBfnQE6f79nVcAq2wci0mGgnjXtwEDAwEAAgjhH5MJ27d7aw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));
    let msg_sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    call_smart_contract(&hex_payload, &msg_sender);
}

#[test]
fn it_should_validate_public_key_eth_matches_solana() {
    setup();
    /*
    Each item in the signatures array is a digital signature of the given message.
    The Solana runtime verifies that the number of signatures matches the number
    in the first 8 bits of the message header.
    It also verifies that each signature was signed by the private key corresponding
    to the public key at the same index in the message's account addresses array.
     */
    let encoded64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAIFAAAAAAAAAAAAAAAAZiK5/895coK4as709oitGuXWn/M9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW79hOdgffqSFwR4MaqpqSzXlBSS0M5PrBq8FFzX/hnMW5AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUd2Jd/lKEoTVTiIYaxgO2ii/8ZHEybZ1ho7bLWzqpLo3AQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));
    let msg_sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    call_smart_contract(&hex_payload, &msg_sender);
}

#[test]
fn it_should_call_adapter_to_read_account() {
    setup();
    create_default_account();
    let pubkey = "58tPH4GsdSn5ehKszkcWK12S2rTBcG9GaMfKtkEZDBKt";
    let account_data = read_account_info_as_json(&pubkey);
    println!("{:?}", account_data);
}

#[test]
fn it_should_decode() {
    let payload = format!(
        "0x{}",
        hex::encode("58tPH4GsdSn5ehKszkcWK12S2rTBcG9GaMfKtkEZDBKt")
    );
    let hex_decoded = hex::decode(&payload[2..]).unwrap();
    let pubkey_str = std::str::from_utf8(&hex_decoded).unwrap();
    println!("read pubkey {}", pubkey_str);
}
