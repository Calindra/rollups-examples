use std::io;

use stdio_call::call_smart_contract_base64;

fn main() -> io::Result<()> {
    let mut msg_sender = String::new();
    io::stdin().read_line(&mut msg_sender)?;
    let mut payload = String::new();
    io::stdin().read_line(&mut payload)?;
    call_smart_contract_base64(
        &payload[..(&payload.len() - 1)],
        &msg_sender[..(&msg_sender.len() - 1)],
    );
    Ok(())
}
