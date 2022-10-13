use std::{rc::Rc, cell::RefCell, str::FromStr};
use ctsi_sol::{AccountManager, AccountFileData};
use serde::{Deserialize, Serialize};
use ctsi_sol::anchor_lang::{prelude::{Pubkey, AccountInfo}};
use json::{object, JsonValue};
use transaction::Signature;
pub mod transaction;

async fn print_response<T: hyper::body::HttpBody>(
    response: hyper::Response<T>,
) -> Result<(), Box<dyn std::error::Error>>
where
    <T as hyper::body::HttpBody>::Error: 'static,
    <T as hyper::body::HttpBody>::Error: std::error::Error,
{
    let response_status = response.status().as_u16();
    let response_body = hyper::body::to_bytes(response).await?;
    println!(
        "Received notice status {} body {}",
        response_status,
        std::str::from_utf8(&response_body)?
    );
    Ok(())
}

fn create_account_manager() -> AccountManager {
    let mut account_manager = AccountManager::new().unwrap();
    let result = std::env::var("SOLANA_DATA_PATH");
    match result {
        Ok(path) => {
            account_manager.set_base_path(path);
            return account_manager;
        },
        Err(_) => {
            account_manager.set_base_path("./".to_owned());
            return account_manager;
        },
    };
}

fn load_account_info_data(pubkey: &Pubkey) -> (Vec<u8>, u64, Pubkey) {
    let account_manager = create_account_manager();
    const MAX_SIZE: usize = 2000;
    let mut lamports = 1000;
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
            let mut info_data = zeroes.to_vec();
            let mut owner: Pubkey = solana_smart_contract::ID;
            if pubkey.to_string() == "6Tw6Z6SsM3ypmGsB3vpSx8midhhyTvTwdPd7K413LyyY" {
                owner = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
                lamports = 0;
                let zeroes: [u8; 165] = [0; 165];
                info_data = zeroes.to_vec();
            }
            if pubkey.to_string() == "4xRtyUw1QSVZSGi1BUb7nbYBk8TC9P1K1AE2xtxwaZmV" {
                println!("Mint not found");
                owner = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
                info_data = vec![
                    1,
                    0,
                    0,
                    0,
                    175,
                    35,
                    124,
                    60,
                    86,
                    42,
                    49,
                    153,
                    12,
                    90,
                    41,
                    181,
                    244,
                    158,
                    219,
                    93,
                    35,
                    126,
                    32,
                    99,
                    96,
                    228,
                    221,
                    154,
                    226,
                    15,
                    253,
                    35,
                    204,
                    138,
                    183,
                    90,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    9,
                    1,
                    1,
                    0,
                    0,
                    0,
                    175,
                    35,
                    124,
                    60,
                    86,
                    42,
                    49,
                    153,
                    12,
                    90,
                    41,
                    181,
                    244,
                    158,
                    219,
                    93,
                    35,
                    126,
                    32,
                    99,
                    96,
                    228,
                    221,
                    154,
                    226,
                    15,
                    253,
                    35,
                    204,
                    138,
                    183,
                    90
                ];
                //info_data = zeroes.to_vec();
            }
            return (info_data, lamports, owner);
        }
    };
}

#[derive(Serialize, Deserialize)]
pub struct AccountJson {
    key: String,
    owner: String,
    data: String,
    lamports: String,
}

pub fn read_account_info_as_json(pubkey_str: &str) -> String {
    let pubkey = Pubkey::from_str(pubkey_str).unwrap();
    let account_manager = create_account_manager();
    let account_file_data = account_manager.read_account(&pubkey).unwrap();
    let account_json = AccountJson {
        key: pubkey_str.to_string(),
        owner: Pubkey::from(account_file_data.owner).to_string(),
        data: base64::encode(account_file_data.data),
        lamports: account_file_data.lamports.to_string()
    };
    serde_json::to_string(&account_json).unwrap()
}

fn check_signature(pubkey: &Pubkey, sender_bytes: &[u8], _signature: &Signature) -> bool {
    sender_bytes == &pubkey.to_bytes()[12..]
}

pub fn call_smart_contract(payload: &str, msg_sender: &str) {
    let encoded64 = hex::decode(&payload[2..]).unwrap();
    let decoded = base64::decode(encoded64).unwrap();
    let tx: transaction::Transaction = bincode::deserialize(&decoded).unwrap();
    let sender_bytes: Vec<u8> = hex::decode(&msg_sender[2..]).unwrap().into_iter().rev().collect();

    let program_id = solana_smart_contract::ID;
    for tx_instruction in &tx.message.instructions {
        let mut accounts = Vec::new();
        let mut params = Vec::new();
        let mut i = 0;
        for pubkey in tx.message.account_keys.iter() {
            let (a, b, c) = load_account_info_data(&pubkey);
            let mut is_signer = false;
            if tx.signatures.len() > i {
                let signature = &tx.signatures[i];
                is_signer = check_signature(&pubkey, &sender_bytes, &signature);
            }
            params.push((a, b, c, pubkey, is_signer));
            i = i + 1;
        }
        for param in params.iter_mut() {
            let key = &param.3;
            let is_signer = &param.4;
            let is_writable = true;
            let executable = true;
            let account_info = AccountInfo {
                key,
                is_signer: *is_signer,
                is_writable,
                lamports: Rc::new(RefCell::new(&mut param.1)),
                data: Rc::new(RefCell::new(&mut param.0)),
                owner: &param.2,
                executable,
                rent_epoch: 1,
            };
            accounts.push(account_info);
        }

        let pidx: usize = (tx_instruction.program_id_index).into();
        println!("tx_instruction.program_id_index = {:?}", tx_instruction.program_id_index);
        println!("tx_instruction.program_id = {:?}", accounts[pidx].key);
        println!("tx.message.header.num_required_signatures = {:?}", tx.message.header.num_required_signatures);
        println!("tx.message.header.num_readonly_signed_accounts = {:?}", tx.message.header.num_readonly_signed_accounts);
        println!("signatures.len() = {:?}", tx.signatures.len());
        println!("accounts indexes = {:?}", tx_instruction.accounts);
        println!("method dispatch's sighash = {:?}", &tx_instruction.data[..8]);
        let mut ordered_accounts = Vec::new();
        for index in tx_instruction.accounts.iter() {
            let i: usize = (*index).into();
            ordered_accounts.push(accounts[i].to_owned());
        }
        for acc in ordered_accounts.iter() {
            println!("- ordered_accounts = {:?}", acc.key);
        }
        solana_smart_contract::entry(&program_id, &ordered_accounts, &tx_instruction.data).unwrap();
        let account_manager = create_account_manager();
        for acc in ordered_accounts.iter() {
            let data = acc.data.borrow_mut();
            let lamports: u64 = **acc.lamports.borrow_mut();
            let account_file_data = AccountFileData {
                owner: acc.owner.to_owned(),
                data: data.to_vec(),
                lamports: lamports,
            };
            if lamports <= 0 {
                account_manager.delete_account(&acc.key).unwrap();
                println!("! deleted = {:?}", acc.key);
            } else {
                account_manager.write_account(&acc.key, &account_file_data).unwrap();
                println!("   saved = {:?}", acc.key);
            }
        }
    }
}

pub async fn handle_advance(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received advance request data {}", &request);
    let payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;
    let msg_sender = request["data"]["metadata"]["msg_sender"]
    .as_str()
    .ok_or("Missing msg_sender")?;
    println!("Adding notice");
    call_smart_contract(&payload, &msg_sender);
    let notice = object! {"payload" => format!("{}", payload)};
    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!("{}/notice", server_addr))
        .body(hyper::Body::from(notice.dump()))?;
    let response = client.request(req).await?;
    print_response(response).await?;
    Ok("accept")
}

pub async fn handle_inspect(
    client: &hyper::Client<hyper::client::HttpConnector>,
    server_addr: &str,
    request: JsonValue,
) -> Result<&'static str, Box<dyn std::error::Error>> {
    println!("Received inspect request data {}", &request);
    let payload = request["data"]["payload"]
        .as_str()
        .ok_or("Missing payload")?;
    println!("Adding report");

    // baby step: just read account info
    let hex_decoded = hex::decode(&payload[2..]).unwrap();
    let pubkey_str = std::str::from_utf8(&hex_decoded).unwrap();
    println!("read pubkey {}", pubkey_str);
    let account_data = read_account_info_as_json(&pubkey_str);
    let report = object! {"payload" => format!("0x{}", hex::encode(account_data))};

    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!("{}/report", server_addr))
        .body(hyper::Body::from(report.dump()))?;
    let response = client.request(req).await?;
    print_response(response).await?;
    Ok("accept")
}
