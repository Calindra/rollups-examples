use std::io;

use base_spl::entry;
use cartesi_solana::adapter::call_solana_program;


fn main() -> io::Result<()> {
    call_solana_program(entry)?;
    Ok(())
}
