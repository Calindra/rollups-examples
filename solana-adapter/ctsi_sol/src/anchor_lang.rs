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
    pub use ::anchor_lang::solana_program::*;

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

        // // pub fn create_account_with_seed<'a, 'b, 'c, 'info>(
        // //     ctx: CpiContext<'a, 'b, 'c, 'info, CreateAccountWithSeed<'info>>,
        // //     seed: &str,
        // //     lamports: u64,
        // //     space: u64,
        // //     owner: &Pubkey,
        // // ) -> Result<()> {
        // //     println!("create_account_with_seed");

        // //     let ix = solana_program::system_instruction::create_account_with_seed(
        // //         ctx.accounts.from.key,
        // //         ctx.accounts.to.key,
        // //         ctx.accounts.base.key,
        // //         seed,
        // //         lamports,
        // //         space,
        // //         owner,
        // //     );
        // //     solana_program::program::invoke_signed(
        // //         &ix,
        // //         &[ctx.accounts.from, ctx.accounts.to, ctx.accounts.base],
        // //         ctx.signer_seeds,
        // //     )
        // //     .map_err(Into::into)
        // //     // Ok(())
        // // }

        // pub fn assign<'a, 'b, 'c, 'info>(
        //     ctx: CpiContext<'a, 'b, 'c, 'info, Assign<'info>>,
        //     owner: &Pubkey,
        // ) -> Result<()> {
        //     println!("assign");
        //     // let ix = crate::solana_program::system_instruction::assign(
        //     //     ctx.accounts.account_to_assign.key,
        //     //     owner,
        //     // );
        //     // crate::solana_program::program::invoke_signed(
        //     //     &ix,
        //     //     &[ctx.accounts.account_to_assign],
        //     //     ctx.signer_seeds,
        //     // )
        //     // .map_err(Into::into)
        //     Ok(())
        // }

        // pub fn transfer<'a, 'b, 'c, 'info>(
        //     ctx: CpiContext<'a, 'b, 'c, 'info, Transfer<'info>>,
        //     lamports: u64,
        // ) -> Result<()> {
        //     println!("Transfer...");
        //     // let ix = crate::solana_program::system_instruction::transfer(
        //     //     ctx.accounts.from.key,
        //     //     ctx.accounts.to.key,
        //     //     lamports,
        //     // );
        //     // crate::solana_program::program::invoke_signed(
        //     //     &ix,
        //     //     &[ctx.accounts.from, ctx.accounts.to],
        //     //     ctx.signer_seeds,
        //     // )
        //     // .map_err(Into::into)
        //     Ok(())
        // }
    }
}

#[cfg(not(target_arch = "bpf"))]
pub mod prelude {
    pub use ::anchor_lang::prelude::*;
    use ::anchor_lang::solana_program::sysvar::SysvarId;
    use serde::{Deserialize, Serialize};

    pub struct Clock {
        pub unix_timestamp: i64,
    }

    impl Clock {
        pub fn get() -> Result<Clock> {
            Ok(Clock {
                unix_timestamp: 123,
            })
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
