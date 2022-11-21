use json::{object, JsonValue};

pub fn create_mint_nft_payload(to_address: &str, token_uri: &str) -> String {
    #[cfg(not(target_arch = "bpf"))]
    {
        _private::create_mint_nft_payload(to_address, token_uri)
    }
    #[cfg(target_arch = "bpf")]
    {
        "0x".to_string()
    }
}

pub fn create_erc20_voucher(recipient: &str, amount: &u64) -> String {
    #[cfg(not(target_arch = "bpf"))]
    {
        return _private::create_erc20_voucher(recipient, amount);
    }
    #[cfg(target_arch = "bpf")]
    {
        "0x".to_string()
    }
}

const HEADER_SIZE: usize = 32;
const AMOUNT_SIZE: usize = 8;
const ADDRESS_SIZE: usize = 20;

pub fn decode_erc20_amount(payload: &str) -> u64 {
    let ini = HEADER_SIZE * 2 + 2; // header has 32 bytes
    let hex_str = &payload[ini..(ini + AMOUNT_SIZE * 2)];
    let bytes: [u8; 8] = hex::decode(hex_str).unwrap().try_into().unwrap();
    let res: u64 = u64::from_be_bytes(bytes);
    res
}

pub fn decode_erc20_address(payload: &str) -> String {
    let ini = (HEADER_SIZE + AMOUNT_SIZE) * 2 + 2; // header has 32 bytes
    let hex_str = &payload[ini..(ini + ADDRESS_SIZE * 2)];
    hex_str.to_owned()
}

pub fn process_erc20(payload: &str, msg_sender: &str) -> JsonValue {
    let amount = decode_erc20_amount(&payload);
    let smart_contract_address = decode_erc20_address(&payload);
    let erc20_transfer_payload = create_erc20_voucher(&msg_sender, &amount);
    //erc20_transfer_payload
    object! {
        address: format!("0x{}", smart_contract_address),
        payload: erc20_transfer_payload,
    }
}

#[cfg(not(target_arch = "bpf"))]
mod _private {
    use ethabi::{
        ethereum_types::{H160, U256},
        Function, Param, ParamType, StateMutability, Token,
    };
    use regex::Regex;

    pub(crate) fn create_erc20_voucher(recipient: &str, amount: &u64) -> String {
        let recipient_param = Param {
            name: "recipient".to_owned(),
            kind: ParamType::Address,
            internal_type: None,
        };
        let token_uri_param = Param {
            name: "amount".to_owned(),
            kind: ParamType::Uint(256),
            internal_type: None,
        };
        let out_bool = Param {
            name: "".to_owned(),
            kind: ParamType::Bool,
            internal_type: None,
        };
        let inputs = vec![recipient_param, token_uri_param];
        let outputs = vec![out_bool];
        let state_mutability = StateMutability::NonPayable;
        let ethfun = Function {
            name: "transfer".to_owned(),
            inputs,
            outputs,
            state_mutability,
            constant: None,
        };
        let re = Regex::new(r"^0x").unwrap();
        let to_address = re.replace_all(recipient, "").to_string();
        let addr_hex: [u8; 20] = hex::decode(to_address).unwrap().try_into().unwrap();
        let addr = H160(addr_hex);
        let address = Token::Address(addr);
        let u256: U256 = U256::from_dec_str(&amount.to_string()).unwrap();
        let amount_u256 = Token::Uint(u256);
        let tokens = vec![address, amount_u256];
        let bytes = ethfun.encode_input(&tokens).unwrap();
        let result = hex::encode(&bytes);
        format!("0x{}", result)
    }

    pub(crate) fn create_mint_nft_payload(to_address: &str, token_uri: &str) -> String {
        let recipient_param = Param {
            name: "recipient".to_owned(),
            kind: ParamType::Address,
            internal_type: None,
        };
        let token_uri_param = Param {
            name: "_tokenURI".to_owned(),
            kind: ParamType::String,
            internal_type: None,
        };
        let out_uint256 = Param {
            name: "".to_owned(),
            kind: ParamType::Uint(256),
            internal_type: None,
        };
        let inputs = vec![recipient_param, token_uri_param];
        let outputs = vec![out_uint256];
        let state_mutability = StateMutability::NonPayable;
        let ethfun = Function {
            name: "mintNFT".to_owned(),
            inputs,
            outputs,
            state_mutability,
            constant: None,
        };
        let re = Regex::new(r"^0x").unwrap();
        let to_address = re.replace_all(to_address, "").to_string();
        let addr_hex: [u8; 20] = hex::decode(to_address).unwrap().try_into().unwrap();
        let addr = H160(addr_hex);
        let address = Token::Address(addr);
        let token_uri = Token::String(token_uri.to_owned());
        let tokens = vec![address, token_uri];
        let bytes = ethfun.encode_input(&tokens).unwrap();
        let result = hex::encode(&bytes);
        format!("0x{}", result)
    }
}
