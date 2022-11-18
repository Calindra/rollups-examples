use solana_adapter::voucher;

#[test]
fn it_should_run_ethabi_encode() {
    let addr_hex_str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    let token_uri = "http://mydomain.com";
    let nft_voucher_payload = voucher::create_mint_nft_payload(addr_hex_str, token_uri);

    // this payload works with the portal!
    assert_eq!(nft_voucher_payload, "0xeacabe14000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000013687474703a2f2f6d79646f6d61696e2e636f6d00000000000000000000000000");
}
