use cartesi_solana::owner_manager;
use json::object;
use solana_adapter::{self, call_smart_contract, deposit};
use std::fs;
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
    deposit::only_accepts_deposits_from_address("0xf8c694fd58360de278d5ff2276b7130bfdc0192a".to_string());
    unsafe {
        owner_manager::POINTERS.clear();
        owner_manager::OWNERS.clear();
    }
}

#[test]
fn it_should_detect_errors() {
    setup();
    let encoded64 = "AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAUHAAAAAAAAAAAAAAAAZiK5/895coK4as709oitGuXWn/NRMLOxTZKDRoTgckYGMm7zeuFm6Fu9cFsRR2oRHd3NhwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFMuB2+5klhUE22BCHIc0cmVUS5eCi0BLmohvoVZCr1E6xifHi/mz/JHBouMDsoFNr79q7lwcL00uX4UZJehrvAan1RcZLFxRIYzJTD1K8X9Y2u4Im6H9ROPb2YoAAAAABt324ddloZPZy+FGzut5rBy0he1fWzeROoz1hX7/AKlxT484t040ZNvBoIHu+yWQcu7cGELHigZMqXbq2/DbkAEDBgEABAYCBQiNhOmCqLcKdw==";
    let hex_payload = format!("0x{}", hex::encode(encoded64));
    // let msg_sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    let wrong_msg_sender = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb9aaaa";
    let timestamp = "1668006843";
    let resp = call_smart_contract(&hex_payload, &wrong_msg_sender, &timestamp);
    let got_an_error = match resp {
        Ok(_) => panic!("This should return an error!"),
        Err(_) => {
            println!("*********************************************");
            println!("********** this error is expected! **********");
            println!("*********************************************");
            true
        },
    };
    assert!(got_an_error);
}

#[tokio::test]
async fn it_should_create_a_token_account_calling_handle_advance() {
    setup();
    let rollup_server_addr = "";
    let client = hyper::Client::new();
    let request = object!{"request_type":"advance_state","data":{"metadata":{"msg_sender":"0xf8c694fd58360de278d5ff2276b7130bfdc0192a","epoch_index":0,"input_index":1,"block_number":43,"timestamp":1668721036},"payload":"0x59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000067d269191c92caf3cd7723f116c85e6e9bf5593300000000000000000000000000000000000000000000000000000000000004d200000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000"}};
    let result = solana_adapter::handle_advance(&client, rollup_server_addr, request).await.unwrap();
    assert_eq!(result, "accept");
}
