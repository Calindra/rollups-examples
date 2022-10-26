use std::io;

use ctsi_sol::adapter::call_solana_program;
use solana_smart_contract::entry;


fn main() -> io::Result<()> {
    call_solana_program(entry)?;
    Ok(())
}
