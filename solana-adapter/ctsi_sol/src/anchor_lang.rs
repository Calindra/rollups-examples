pub use anchor_lang::*;

// Replacing the
// anchor_lang::system_program::create_account
#[cfg(not(target_arch = "bpf"))]
pub mod system_program {
    use anchor_lang::prelude::{CpiContext, Pubkey, Result};
    // use anchor_lang::solana_program;
    use crate::owner_manager;
    pub use anchor_lang::system_program::*;

    pub fn create_account<'a, 'b, 'c, 'info>(
        ctx: CpiContext<'a, 'b, 'c, 'info, CreateAccount<'info>>,
        _lamports: u64,
        _space: u64,
        owner: &Pubkey,
    ) -> Result<()> {
        anchor_lang::prelude::msg!("Inside lang system_program create_account...");
        owner_manager::change_owner(ctx.accounts.to.key.clone(), owner.clone());
        // TODO: check the correct amount
        **ctx.accounts.to.try_borrow_mut_lamports()? += 100000;

        // let ix = solana_program::system_instruction::create_account(
        //     ctx.accounts.from.key,
        //     ctx.accounts.to.key,
        //     lamports,
        //     space,
        //     owner,
        // );
        // solana_program::program::invoke_signed(
        //     &ix,
        //     &[ctx.accounts.from, ctx.accounts.to],
        //     ctx.signer_seeds,
        // )
        // .map_err(Into::into)
        Ok(())
    }
}

#[cfg(not(target_arch = "bpf"))]
pub mod solana_program {
    pub use anchor_lang::solana_program::*;

    // anchor_lang::solana_program::program::invoke_signed
    pub mod program {
        pub use anchor_lang::solana_program::program::*;
        use anchor_lang::{
            prelude::AccountInfo,
            solana_program::{entrypoint::ProgramResult, instruction::Instruction},
        };
        pub fn invoke_signed(
            instruction: &Instruction,
            account_infos: &[AccountInfo],
            _signers_seeds: &[&[&[u8]]],
        ) -> ProgramResult {
            anchor_lang::prelude::msg!("anchor_lang::solana_program::program::invoke_signed...");
            // Check that the account RefCells are consistent with the request
            for account_meta in instruction.accounts.iter() {
                for account_info in account_infos.iter() {
                    if account_meta.pubkey == *account_info.key {
                        if account_meta.is_writable {
                            let _ = account_info.try_borrow_mut_lamports()?;
                            let _ = account_info.try_borrow_mut_data()?;
                        } else {
                            let _ = account_info.try_borrow_lamports()?;
                            let _ = account_info.try_borrow_data()?;
                        }
                        break;
                    }
                }
            }

            //invoke_signed_unchecked(instruction, account_infos, signers_seeds)
            Ok(())
        }
    }
    pub mod system_instruction {
        use anchor_lang::prelude::Result;
        pub use anchor_lang::solana_program::system_instruction::*;
        use anchor_lang::{
            prelude::{CpiContext, Pubkey},
            solana_program,
            system_program::{Allocate, CreateAccount},
        };

        pub fn allocate<'a, 'b, 'c, 'info>(
            ctx: CpiContext<'a, 'b, 'c, 'info, Allocate<'info>>,
            space: u64,
        ) -> Result<()> {
            println!("allocate");

            let ix = solana_program::system_instruction::allocate(
                ctx.accounts.account_to_allocate.key,
                space,
            );
            solana_program::program::invoke_signed(
                &ix,
                &[ctx.accounts.account_to_allocate],
                ctx.signer_seeds,
            )
            .map_err(Into::into)
        }

        pub fn create_account_x<'a, 'b, 'c, 'info>(
            ctx: CpiContext<'a, 'b, 'c, 'info, CreateAccount<'info>>,
            lamports: u64,
            space: u64,
            owner: &Pubkey,
        ) -> Result<()> {
            anchor_lang::prelude::msg!("Inside create_account?...");
            let ix = ::anchor_lang::solana_program::system_instruction::create_account(
                ctx.accounts.from.key,
                ctx.accounts.to.key,
                lamports,
                space,
                owner,
            );
            crate::anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[ctx.accounts.from, ctx.accounts.to],
                ctx.signer_seeds,
            )
            .map_err(Into::into)
        }
    }
}

#[cfg(not(target_arch = "bpf"))]
pub mod prelude {
    pub use anchor_lang::prelude::*;
    use anchor_lang::solana_program::sysvar::SysvarId;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    pub struct Clock {
        pub unix_timestamp: i64,
    }

    impl Clock {
        pub fn get() -> Result<Clock> {
            let time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let unix_timestamp = i64::from_str(&time.to_string()).unwrap();
            Ok(Clock { unix_timestamp })
        }
    }

    #[derive(Default, Serialize, Deserialize)]
    pub struct Rent {}
    impl Rent {
        pub fn get() -> core::result::Result<anchor_lang::prelude::Rent, ProgramError> {
            Ok(anchor_lang::prelude::Rent {
                lamports_per_byte_year: 1,
                exemption_threshold: 0.1,
                burn_percent: 1,
            })
        }
    }
    impl SysvarId for Rent {
        fn id() -> Pubkey {
            Pubkey::default()
        }
        fn check_id(_: &Pubkey) -> bool {
            true
        }
    }

    impl<'a, 'b> anchor_lang::prelude::SolanaSysvar for Rent {}
}
