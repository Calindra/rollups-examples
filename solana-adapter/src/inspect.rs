use std::{collections::HashMap, str::FromStr};

use cartesi_solana::account_manager::{create_account_manager, AccountFileData};
use json::{object, JsonValue};
use solana_program::pubkey::Pubkey;
use spl_token::state::{Account, GenericTokenAccount};
use url::Url;

use crate::{print_response, AccountJson};

static TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";

pub async fn handle(
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
    let path_str = std::str::from_utf8(&hex_decoded).unwrap();
    println!("inspect decoded command {}", path_str);
    if path_str.starts_with("programAccounts/") {
        let jsons = find_program_accounts(&path_str[16..]);
        match jsons {
            Ok(each_jsons) => {
                for account_data in each_jsons.iter() {
                    let report = object! {"payload" => format!("0x{}", hex::encode(account_data))};

                    let req = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .header(hyper::header::CONTENT_TYPE, "application/json")
                        .uri(format!("{}/report", server_addr))
                        .body(hyper::Body::from(report.dump()))?;
                    let response = client.request(req).await?;
                    print_response(response).await?;
                }
                Ok("accept")
            }
            Err(_) => Ok("reject"),
        }
    } else if path_str.starts_with("accountInfo/") {
        let account_data_res = read_account_info_as_json_string(&path_str[12..]);
        match account_data_res {
            Ok(account_data) => {
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
            Err(_) => Ok("reject"),
        }
    } else if path_str.starts_with("tokenAccountsByOwner/") {
        let jsons = handle_token_accounts_by_owner(&path_str[21..]);
        match jsons {
            Ok(each_jsons) => {
                for account_data in each_jsons.iter() {
                    let report = object! {"payload" => format!("0x{}", hex::encode(account_data))};

                    let req = hyper::Request::builder()
                        .method(hyper::Method::POST)
                        .header(hyper::header::CONTENT_TYPE, "application/json")
                        .uri(format!("{}/report", server_addr))
                        .body(hyper::Body::from(report.dump()))?;
                    let response = client.request(req).await?;
                    print_response(response).await?;
                }
                Ok("accept")
            }
            Err(_) => Ok("reject"),
        }
    } else {
        let account_data_res = read_account_info_as_json_string(&path_str);
        match account_data_res {
            Ok(account_data) => {
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
            Err(_) => Ok("reject"),
        }
    }
}

pub fn find_program_accounts(pubkey_str: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pubkey = Pubkey::from_str(pubkey_str).expect(&format!("Pubkey error: {}", pubkey_str));
    let account_manager = create_account_manager();
    let accounts = account_manager.find_program_accounts(&pubkey)?;
    let res = accounts.iter().map(stringify).collect();
    Ok(res)
}

pub fn read_account_info_as_json_string(
    pubkey_str: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let pubkey = Pubkey::from_str(pubkey_str).unwrap();
    let account_manager = create_account_manager();
    let account_file_data = account_manager.read_account(&pubkey)?;
    Ok(stringify(&(pubkey, account_file_data)))
}

pub fn handle_token_accounts_by_owner(
    url_path: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = Url::parse(&format!("http://cartesi/{}", url_path)).unwrap();
    let pubkey = &url.path()[1..];
    let pairs: HashMap<_, _> = url.query_pairs().into_owned().collect();
    let has_mint = pairs.get("mint");
    match has_mint {
        Some(mint) => get_token_accounts_by_owner_and_mint(pubkey, &mint),
        None => get_token_accounts_by_owner(pubkey),
    }
}

pub fn get_token_accounts_by_owner_and_mint(
    pubkey: &str,
    mint: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let token_owner = Pubkey::from_str(pubkey).unwrap();
    let token_accounts_owner_program_id = Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap();
    let mint = Pubkey::from_str(mint).unwrap();
    let account_manager = create_account_manager();
    let all_token_accounts =
        account_manager.find_program_accounts(&token_accounts_owner_program_id)?;
    let owned: Vec<String> = all_token_accounts
        .iter()
        .filter(|(_, account)| is_owned_by(&token_owner, account))
        .filter(|(_, account)| is_account_mint(&mint, account))
        .map(stringify)
        .collect();
    return Ok(owned);
}

pub fn get_token_accounts_by_owner(
    pubkey: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let token_owner = Pubkey::from_str(pubkey).unwrap();
    let token_accounts_owner_program_id = Pubkey::from_str(TOKEN_PROGRAM_ID).unwrap();
    let account_manager = create_account_manager();
    let all_token_accounts =
        account_manager.find_program_accounts(&token_accounts_owner_program_id)?;
    let owned: Vec<String> = all_token_accounts
        .iter()
        .filter(|(_, account)| is_owned_by(&token_owner, account))
        .map(stringify)
        .collect();
    return Ok(owned);
}

fn stringify(tup: &(Pubkey, AccountFileData)) -> String {
    let account_json = to_account_json(tup);
    serde_json::to_string(&account_json).expect("Fail to stringify the account")
}

fn to_account_json((pubkey, account_file_data): &(Pubkey, AccountFileData)) -> AccountJson {
    AccountJson {
        key: pubkey.to_string(),
        owner: Pubkey::from(account_file_data.owner).to_string(),
        data: base64::encode(&account_file_data.data),
        lamports: account_file_data.lamports.to_string(),
    }
}

fn is_owned_by(token_owner: &Pubkey, account: &AccountFileData) -> bool {
    let owner = Account::unpack_account_owner(&account.data);
    match owner {
        Some(owner) => owner == token_owner,
        None => false,
    }
}

fn is_account_mint(mint: &Pubkey, account: &AccountFileData) -> bool {
    let data_mint = Account::unpack_account_mint(&account.data);
    match data_mint {
        Some(data_mint) => data_mint == mint,
        None => false,
    }
}
