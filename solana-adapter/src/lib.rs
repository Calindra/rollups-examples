use ctsi_sol::account_manager::create_account_manager;
use ctsi_sol::anchor_lang::prelude::Pubkey;
use json::{object, JsonValue};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::process::{Command, Stdio};
use std::str::FromStr;

pub mod deposit;
pub mod voucher;

pub mod transaction;

static ERC20_TRANSFER_HEADER: &str =
    "59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378";

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
        "Received notice status[{}] body[{}]",
        response_status,
        std::str::from_utf8(&response_body)?
    );
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub struct AccountJson {
    key: String,
    owner: String,
    data: String,
    lamports: String,
}

pub fn find_program_accounts(pubkey_str: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pubkey = Pubkey::from_str(pubkey_str).unwrap();
    let account_manager = create_account_manager();
    let accounts = account_manager.find_program_accounts(&pubkey)?;
    let mut res = vec![];
    for (key, account_file_data) in accounts.iter() {
        let account_json = AccountJson {
            key: key.to_string(),
            owner: Pubkey::from(account_file_data.owner).to_string(),
            data: base64::encode(&account_file_data.data),
            lamports: account_file_data.lamports.to_string(),
        };
        res.push(serde_json::to_string(&account_json).unwrap());
    }
    Ok(res)
}

pub fn read_account_info_as_json(pubkey_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let pubkey = Pubkey::from_str(pubkey_str).unwrap();
    let account_manager = create_account_manager();
    let account_file_data = account_manager.read_account(&pubkey)?;

    let account_json = AccountJson {
        key: pubkey_str.to_string(),
        owner: Pubkey::from(account_file_data.owner).to_string(),
        data: base64::encode(account_file_data.data),
        lamports: account_file_data.lamports.to_string(),
    };
    Ok(serde_json::to_string(&account_json).unwrap())
}

#[derive(Debug)]
pub enum ContractError {
    UnexpectedError,
}
impl std::error::Error for ContractError {}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContractError::UnexpectedError => write!(f, "Contract Error"),
        }
    }
}

pub fn call_smart_contract(
    payload: &str,
    msg_sender: &str,
    timestamp: &str,
) -> Result<(), ContractError> {
    let encoded64 = hex::decode(&payload[2..]).unwrap();
    let decoded = base64::decode(&encoded64).unwrap();
    let tx: transaction::Transaction = bincode::deserialize(&decoded).unwrap();
    let mut instruction_index: usize = 0;
    for tx_instruction in &tx.message.instructions {
        let pidx: usize = (tx_instruction.program_id_index).into();
        let program_id = tx.message.account_keys[pidx];
        println!("program_id = {:?}", program_id);
        let mut child = Command::new(format!("./solana_smart_contract_bin/{:?}", program_id))
            .stdin(Stdio::piped())
            .spawn()
            .unwrap();
        let child_stdin = child.stdin.as_mut().unwrap();
        child_stdin.write_all(msg_sender.as_bytes()).unwrap();
        child_stdin.write_all(b"\n").unwrap();
        child_stdin.write_all(&encoded64).unwrap();
        child_stdin.write_all(b"\n").unwrap();
        child_stdin
            .write_all(&instruction_index.to_string().as_bytes())
            .unwrap();
        child_stdin.write_all(b"\n").unwrap();
        child_stdin.write_all(timestamp.as_bytes()).unwrap();
        child_stdin.write_all(b"\n").unwrap();

        drop(child_stdin);
        let output = child.wait_with_output().unwrap();
        println!("output = {:?}", output);
        let opt_exit_code = output.status.code();
        match opt_exit_code {
            Some(exit_code) => {
                if exit_code == 0 {
                    println!("Success!!!")
                } else {
                    return Err(ContractError::UnexpectedError);
                }
            }
            None => {
                println!("Unexpected error");
                return Err(ContractError::UnexpectedError);
            }
        }
        instruction_index += 1;
    }
    Ok(())
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
    let timestamp = request["data"]["metadata"]["timestamp"]
        .as_i64()
        .ok_or("Missing timestamp")?;
    if &payload[2..66] == ERC20_TRANSFER_HEADER {
        deposit::create_token_account(&payload, &msg_sender, &timestamp.to_string());
        send_report_ok(server_addr, client).await?;
        return Ok("accept");
    }
    let contract_response = call_smart_contract(&payload, &msg_sender, &timestamp.to_string());
    match contract_response {
        Ok(_) => {
            println!("Sending voucher");
            let smart_contract_address = "0xE6E340D132b5f46d1e472DebcD681B2aBc16e57E";
            let to_address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
            let token_uri = "http://mydomain.com/nft";
            let nft_mint_payload = voucher::create_mint_nft_payload(to_address, token_uri);
            let voucher = object! {
                address: smart_contract_address,
                payload: nft_mint_payload,
            };
            let req = hyper::Request::builder()
                .method(hyper::Method::POST)
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .uri(format!("{}/voucher", server_addr))
                .body(hyper::Body::from(voucher.dump()))?;
            let response = client.request(req).await?;
            print_response(response).await?;

            send_report_ok(server_addr, client).await?;
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
        Err(_error) => {
            // TODO: retornar um erro detalhado
            println!("Sending error report!");
            let error_details = hex::encode("{\"error\":1}");
            let notice = object! {"payload" => format!("0x{}", error_details)};
            let req = hyper::Request::builder()
                .method(hyper::Method::POST)
                .header(hyper::header::CONTENT_TYPE, "application/json")
                .uri(format!("{}/report", server_addr))
                .body(hyper::Body::from(notice.dump()))?;
            let response = client.request(req).await?;
            print_response(response).await?;
            Ok("reject")
        }
    }
}

async fn send_report_ok(server_addr: &str, client: &hyper::Client<hyper::client::HttpConnector>) -> Result<(), Box<dyn Error>> {
    if server_addr == "" {
        // just ignore if there is no rollup server
        return Ok(())
    }
    println!("Sending ok report!");
    let ok_result = hex::encode("{\"ok\":1}");
    let notice = object! {"payload" => format!("0x{}", ok_result)};
    let req = hyper::Request::builder()
        .method(hyper::Method::POST)
        .header(hyper::header::CONTENT_TYPE, "application/json")
        .uri(format!("{}/report", server_addr))
        .body(hyper::Body::from(notice.dump()))?;
    let response = client.request(req).await?;
    print_response(response).await?;
    Ok(())
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
    println!("inspect decoded command {}", pubkey_str);
    if pubkey_str.starts_with("programAccounts/") {
        let jsons = find_program_accounts(&pubkey_str[16..]);
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
    } else if pubkey_str.starts_with("accountInfo/") {
        let account_data_res = read_account_info_as_json(&pubkey_str[12..]);
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
    } else {
        let account_data_res = read_account_info_as_json(&pubkey_str);
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
