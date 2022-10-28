use std::io;

use base_spl::entry;
use ctsi_sol::adapter::call_solana_program;


fn main() -> io::Result<()> {
    call_solana_program(entry)?;
    Ok(())
}
