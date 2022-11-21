use std::{time::{SystemTime, UNIX_EPOCH}, fs, cell::RefCell, rc::Rc};

use ctsi_sol::{owner_manager, account_manager::create_account_manager, anchor_lang, anchor_spl::token::TokenAccount};
use solana_adapter::{voucher, deposit};
use solana_program::account_info::AccountInfo;

fn setup() {
    println!("\n\n***** setup *****\n");
    let dir = std::env::temp_dir();
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let final_temp_dir = format!(
        "{}/{}",
        dir.as_os_str().to_str().unwrap(),
        since_the_epoch.subsec_nanos()
    );
    println!("{}", final_temp_dir);
    fs::create_dir(&final_temp_dir).unwrap();
    std::env::set_var("SOLANA_DATA_PATH", final_temp_dir);
    std::env::set_var("PORTAL_ADDRESS", "0xf8c694fd58360de278d5ff2276b7130bfdc0192a");
    unsafe {
        owner_manager::POINTERS.clear();
        owner_manager::OWNERS.clear();
    }
}

#[test]
fn it_should_run_ethabi_encode() {
    let addr_hex_str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    let token_uri = "http://mydomain.com";
    let nft_voucher_payload = voucher::create_mint_nft_payload(addr_hex_str, token_uri);

    // this payload works with the portal!
    assert_eq!(nft_voucher_payload, "0xeacabe14000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000013687474703a2f2f6d79646f6d61696e2e636f6d00000000000000000000000000");
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

#[test]
fn it_should_withdraw_the_amount() {
    setup();
    let deposit_payload = "0x59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000067d269191c92caf3cd7723f116c85e6e9bf5593300000000000000000000000000000000000000000000000000000000000004d200000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
    let msg_sender = "0xf8c694fd58360de278d5ff2276b7130bfdc0192a";
    let timestamp = "1";
    let pubkey = deposit::process(deposit_payload, msg_sender, timestamp);


    // call voucher
    let voucher_payload = "0x0778480ca791e9ab4463d1a02daf76e8a8466940b36135d791d9a92a70e3dc6200000000000000ea67d269191c92caf3cd7723f116c85e6e9bf55933";
    let msg_sender = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    voucher::process_erc20(voucher_payload, msg_sender);

    let account_manager = create_account_manager();
    let account_info_data = account_manager.read_account(&pubkey).unwrap();
    let mut lamports = account_info_data.lamports;
    let mut data = account_info_data.data;
    let account_info = AccountInfo {
        key: &pubkey,
        is_signer: false,
        is_writable: false,
        lamports: Rc::new(RefCell::new(&mut lamports)),
        data: Rc::new(RefCell::new(&mut data)),
        owner: &account_info_data.owner,
        executable: false,
        rent_epoch: 0,
    };
    let token_account: anchor_lang::accounts::account::Account<TokenAccount> =
        anchor_lang::accounts::account::Account::try_from_unchecked(&account_info).unwrap();
    
    assert_eq!(token_account.amount, 1000u64);
}

#[test]
#[should_panic]
fn it_should_not_withdraw_the_amount_cuz_insufficient_funds() {
    setup();
    let deposit_payload = "0x59da2a984e165ae4487c99e5d1dca7e04c8a99301be6bc092932cb5d7f034378000000000000000000000000f39fd6e51aad88f6f4ce6ab8827279cfffb9226600000000000000000000000067d269191c92caf3cd7723f116c85e6e9bf5593300000000000000000000000000000000000000000000000000000000000004d200000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000";
    let msg_sender = "0xf8c694fd58360de278d5ff2276b7130bfdc0192a";
    let timestamp = "1";
    deposit::process(deposit_payload, msg_sender, timestamp);


    // call voucher and should panic!
    let voucher_payload = "0x0778480ca791e9ab4463d1a02daf76e8a8466940b36135d791d9a92a70e3dc62000000003b9aca0067d269191c92caf3cd7723f116c85e6e9bf55933";
    let msg_sender = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
    voucher::process_erc20(voucher_payload, msg_sender);
}