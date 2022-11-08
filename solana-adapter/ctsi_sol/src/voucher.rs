//use ethabi::{ethereum_types::H160, Function, Param, ParamType, StateMutability, Token};
use regex::Regex;


pub fn create_mint_nft_payload(to_address: &str, token_uri: &str) -> String {
    // let recipient_param = Param {
    //     name: "recipient".to_owned(),
    //     kind: ParamType::Address,
    //     internal_type: None,
    // };
    // let token_uri_param = Param {
    //     name: "_tokenURI".to_owned(),
    //     kind: ParamType::String,
    //     internal_type: None,
    // };
    // let out_uint256 = Param {
    //     name: "".to_owned(),
    //     kind: ParamType::Uint(256),
    //     internal_type: None,
    // };
    // let inputs = vec![recipient_param, token_uri_param];
    // let outputs = vec![out_uint256];
    // let state_mutability = StateMutability::NonPayable;
    // let ethfun = Function {
    //     name: "mintNFT".to_owned(),
    //     inputs,
    //     outputs,
    //     state_mutability,
    //     constant: None,
    // };
    // let re = Regex::new(r"^0x").unwrap();
    // let to_address = re.replace_all(to_address, "").to_string();
    // let addr_hex: [u8; 20] = hex::decode(to_address).unwrap().try_into().unwrap();
    // let addr = H160(addr_hex);
    // let address = Token::Address(addr);
    // let token_uri = Token::String(token_uri.to_owned());
    // let tokens = vec![address, token_uri];
    // let bytes = ethfun.encode_input(&tokens).unwrap();
    // let result = hex::encode(&bytes);
    // format!("0x{}", result)
    "0x".to_string()
}
