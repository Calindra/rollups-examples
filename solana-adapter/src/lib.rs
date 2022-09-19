use std::{rc::Rc, cell::RefCell, str::FromStr};
use ctsi_sol::AccountManager;

use anchor_lang::prelude::{Pubkey, AccountInfo};
use json::{object, JsonValue};

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

pub fn call_smart_contract(payload: &str) {
    let encoded64 = hex::decode(&payload[2..]).unwrap();
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