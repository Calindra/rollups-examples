use solana_adapter::{call_smart_contract, self, read_account_info_as_json};

#[test]
fn it_should_call_adapter_without_errors_to_create_an_account() {
    let encoded64 = "AT5FiVtGESwkZI4CSYS3rB1BUKhO/SsuWkdI7U0a+EfOYhWoUFcpPgFDhCa9n6lZP4j/JurMY90/6/PY/XoErA8BAAIFaLXcC6Cywbwm74mPOjeCatSweRxlWr35eTLpIEf+WOE9c+ndk0/3nYBv5IL0AYCdTFv3mclqsrWNe8g7zMKW788smY3PSJVY8mgIeGmx7C+RnzWnx1yuebvR7LVvAwu3AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUy4Hb7mSWFQTbYEIchzRyZVRLl4KLQEuaiG+hVkKvUa4D7xZK8QCXLSfAn7UqVg66AIgQ0PFcKgpkbDEnLEQFAQQEAQIAAzivr20fDZib7awePxMEdMwUsv7P4+23SFy4GOfkeXuwWPj1MMZHa6Xu6AMAAAAAAAAEAAAAc2x1Zw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));

    call_smart_contract(&hex_payload);
}

#[test]
fn it_should_call_adapter_to_update_an_account() {
    
}

#[test]
fn it_should_call_adapter_to_read_account() {
    let pubkey = "58tPH4GsdSn5ehKszkcWK12S2rTBcG9GaMfKtkEZDBKt";
    let account_data = read_account_info_as_json(&pubkey);
    println!("{:?}", account_data);
}

#[test]
fn it_should_decode() {
    let payload = format!("0x{}", hex::encode("58tPH4GsdSn5ehKszkcWK12S2rTBcG9GaMfKtkEZDBKt"));
    let hex_decoded = hex::decode(&payload[2..]).unwrap();
    let pubkey_str = std::str::from_utf8(&hex_decoded).unwrap();
    println!("read pubkey {}", pubkey_str);
}