use std::{rc::Rc, cell::RefCell, str::FromStr};
use ctsi_sol::{AccountManager, AccountFileData};
use serde::{Deserialize, Serialize};
use anchor_lang::{prelude::{Pubkey, AccountInfo}};
use json::{object, JsonValue};
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

pub fn call_smart_contract(payload: &str) {
    let encoded64 = hex::decode(&payload[2..]).unwrap();
    let decoded = base64::decode(encoded64).unwrap();
    let tx: transaction::Transaction = bincode::deserialize(&decoded).unwrap();

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
        println!("- ordered_accounts = {:?}", acc.key);
    }
    solana_smart_contract::entry(&program_id, &ordered_accounts, &first.data).unwrap();
    let account_manager = create_account_manager();
    for acc in ordered_accounts.iter() {
        println!("- saving = {:?}", acc.key);
        let data = acc.data.borrow_mut();
        let lamports: u64 = **acc.lamports.borrow_mut();
        let account_file_data = AccountFileData {
            owner: acc.owner.to_owned(),
            data: data.to_vec(),
            lamports: lamports,
        };
        account_manager.write_account(&acc.key, &account_file_data).unwrap();
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
    println!("Adding notice");
    call_smart_contract(&payload);
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
