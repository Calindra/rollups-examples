use anchor_lang::prelude::Pubkey;
use cartesi_solana::adapter;
use json::{object, JsonValue};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};

pub mod deposit;
pub mod inspect;
pub mod token_account;
pub mod transaction;
pub mod voucher;

static ERC20_TRANSFER_HEADER: &str =
    "59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378";

static ERC20_VOUCHER_HEADER: &str =
    "0778480ca791e9ab4463d1a02daf76e8a8466940b36135d791d9a92a70e3dc62";

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

fn create_child_process(program_id: &Pubkey) -> Child {
    let path = Path::new(&adapter::get_binary_base_path()).join(program_id.to_string());
    if !path.exists() {
        panic!("failed to find program path [{}]", path.display());
    }
    let child = Command::new(&path).stdin(Stdio::piped()).spawn().unwrap();
    child
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
        let mut child = create_child_process(&program_id);
        let child_stdin = child.stdin.as_mut().unwrap();

        child_stdin.write_all(b"Header: External CPI").unwrap();
        child_stdin.write_all(b"\n").unwrap();
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
        deposit::process(&payload, &msg_sender, &timestamp.to_string());
        send_report_ok(server_addr, client).await?;
        return Ok("accept");
    }
    if &payload[2..66] == ERC20_VOUCHER_HEADER {
        println!("Sending erc-20 voucher");
        let voucher = voucher::process_erc20(&payload, &msg_sender);
        let req = hyper::Request::builder()
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .uri(format!("{}/voucher", server_addr))
            .body(hyper::Body::from(voucher.dump()))?;
        let response = client.request(req).await?;
        print_response(response).await?;
        send_report_ok(server_addr, client).await?;
        return Ok("accept");
    }
    let contract_response = call_smart_contract(&payload, &msg_sender, &timestamp.to_string());
    match contract_response {
        Ok(_) => {
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

async fn send_report_ok(
    server_addr: &str,
    client: &hyper::Client<hyper::client::HttpConnector>,
) -> Result<(), Box<dyn Error>> {
    if server_addr == "" {
        // just ignore if there is no rollup server
        return Ok(());
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
