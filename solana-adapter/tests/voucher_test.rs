use solana_adapter::voucher;

#[test]
fn it_should_run_ethabi_encode() {
    let addr_hex_str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    let token_uri = "http://mydomain.com";
    let nft_voucher_payload = voucher::create_mint_nft_payload(addr_hex_str, token_uri);

    // this payload works with the portal!
    assert_eq!(nft_voucher_payload, "0xeacabe14000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000013687474703a2f2f6d79646f6d61696e2e636f6d00000000000000000000000000");
}

#[test]
fn it_should_emit_a_voucher() {
    let payload = "0xa7741b455a9070b02e0d47705b0fe6d1757f00b25a7d3fe4938a26e17c1e610900000000000004d267d269191c92caf3cd7723f116c85e6e9bf55933";

}

#[test]
fn it_should_decode_amount_from_erc20_voucher_payload() {
    let payload = "0x0778480ca791e9ab4463d1a02daf76e8a8466940b36135d791d9a92a70e3dc620000000000bc614e67d269191c92caf3cd7723f116c85e6e9bf55933";
    let amount = voucher::decode_erc20_amount(payload);
    assert_eq!(amount, 12345678u64);
}

#[test]
fn it_should_decode_address_from_erc20_voucher_payload() {
    let payload = "0x0778480ca791e9ab4463d1a02daf76e8a8466940b36135d791d9a92a70e3dc620000000000bc614e67d269191c92caf3cd7723f116c85e6e9bf55933";
    let erc20_smart_contract_adderss = voucher::decode_erc20_address(payload);
    assert_eq!(erc20_smart_contract_adderss.to_lowercase(), "67d269191c92Caf3cD7723F116c85e6E9bf55933".to_lowercase());
}