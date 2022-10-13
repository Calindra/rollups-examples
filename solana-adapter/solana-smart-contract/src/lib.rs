use ctsi_sol::anchor_lang::{self, prelude::*};
use ctsi_sol::anchor_lang::solana_program::system_program;
use ctsi_sol::anchor_spl::token::{self, Transfer, Mint, TokenAccount};
use ctsi_sol::anchor_spl;
use ctsi_sol::Clock;
use ctsi_sol::Rent;


pub mod models;

declare_id!("2QB8wEBJ8jjMQuZPvj3jaZP7JJb5j21u4xbxTnwsZRfv");

#[error_code]
pub enum MyError {
    #[msg("Insuficient amount")]
    InsuficientAmount,

    #[msg("Wrong token")]
    WrongToken,

    #[msg("Wrong owner")]
    WrongOwner,

    #[msg("Wrong parent validation")]
    WrongParentValidation,
}

#[program]
pub mod solzen {
    use super::*;

    const WALLET_PDA_SEED: &[u8] = b"wallet";

    pub fn initialize(
        ctx: Context<InitDAO>,
        token: Pubkey,
        min_balance: u64,
        dao_slug: String,
    ) -> Result<()> {
        msg!("Initializing...");
        let dao = &mut ctx.accounts.zendao;
        let founder: &Signer = &ctx.accounts.founder;
        dao.token = token;
        dao.min_balance = min_balance;
        dao.slug = dao_slug;
        let validation = &mut ctx.accounts.validation;
        validation.child = *founder.key;
        let clock: Clock = Clock::get().unwrap();
        validation.timestamp = clock.unix_timestamp;
        Ok(())
    }

    pub fn init_wallet(ctx: Context<InitWallet>) -> Result<()> {

        // take the ownership of this TokenAccount
        let cpi_accounts = anchor_spl::token::SetAuthority {
            account_or_mint: ctx.accounts.escrow_wallet.to_account_info(),
            current_authority: ctx.accounts.user_sending.to_account_info(),
        };
        let cpi_context = CpiContext::new(ctx.accounts.token_program.clone(), cpi_accounts);
        let (vault_authority, _bump) =
            Pubkey::find_program_address(&[
                WALLET_PDA_SEED,
                ctx.accounts.mint.to_account_info().key.as_ref()
            ], ctx.program_id);
        anchor_spl::token::set_authority(
            cpi_context,
            anchor_spl::token::spl_token::instruction::AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;
        Ok(())
    }

    pub fn transfer(ctx: Context<TransferInstruction>, amount: u64, nonce: u8) -> Result<()> {
        let seeds = &[
            ctx.accounts.mint.to_account_info().key.as_ref(),
            &[nonce],
        ];
        let signer = &[&seeds[..]];
        let cpi_accounts = Transfer {
            from: ctx.accounts.from.to_account_info(),
            to: ctx.accounts.to.to_account_info(),
            authority: ctx.accounts.program_signer.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        msg!("Calling transfer...");
        token::transfer(cpi_ctx, amount).expect("transfer2 failed"); //?;
        msg!("Transfer success.");
        Ok(())
    }

    pub fn update(
        ctx: Context<UpdateDAO>,
        token: Pubkey,
        min_balance: u64,
    ) -> Result<()> {
        msg!("Updating...");
        // TODO: we need to check the founder...
        let _founder: &Signer = &ctx.accounts.founder;

        let dao = &mut ctx.accounts.zendao;
        dao.token = token;
        dao.min_balance = min_balance;
        Ok(())
    }

    pub fn close_dao(_ctx: Context<CloseDAO>) -> Result<()> {
        Ok(())
    }

    pub fn validate_telegram_user(ctx: Context<ValidateTelegramUser>, id: u64) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        let zendao = &ctx.accounts.zendao;
        msg!("telegram id = {:?}", id);
        if token_account.mint.key().to_string() != zendao.token.key().to_string() {
            msg!(
                "wrong mint {:?} should be {:?}",
                token_account.mint.key(),
                zendao.token.key()
            );
            return Err(error!(MyError::WrongToken));
        }
        msg!("token owner {:?}", token_account.owner);
        if token_account.owner != ctx.accounts.signer.key() {
            return Err(error!(MyError::WrongOwner));
        }
        let telegram_user = &mut ctx.accounts.telegram_user;
        if ctx.accounts.token_account.amount < ctx.accounts.zendao.min_balance {
            msg!(
                "token owner = {:?} amount = {:?} min = {:?}",
                token_account.owner,
                ctx.accounts.token_account.amount,
                ctx.accounts.zendao.min_balance
            );
            return Err(error!(MyError::InsuficientAmount));
        }
        telegram_user.pubkey = *ctx.accounts.signer.key;
        telegram_user.id = id;
        telegram_user.dao = ctx.accounts.zendao.key();
        Ok(())
    }

    pub fn validate_human(ctx: Context<ValidateHuman>, child: Pubkey) -> Result<()> {
        let token_account = &ctx.accounts.token_account;
        let zendao = &ctx.accounts.zendao;
        if token_account.mint.key().to_string() != zendao.token.key().to_string() {
            msg!(
                "wrong mint {:?} should be {:?}",
                token_account.mint.key(),
                zendao.token.key()
            );
            return Err(error!(MyError::WrongToken));
        }
        let parent: &Signer = &ctx.accounts.parent;
        msg!(
            "amount = {:?} min_balance = {:?}",
            &token_account.amount,
            zendao.min_balance
        );
        msg!("parent = {:?}", parent.key);
        let parent_validation = &ctx.accounts.parent_validation;
        msg!("parent as child = {:?}", parent_validation.child);
        if parent_validation.child.to_string() != *parent.key.to_string() {
            return Err(error!(MyError::WrongParentValidation));
        }

        if token_account.amount < zendao.min_balance {
            return Err(error!(MyError::InsuficientAmount));
        }
        msg!("owner = {:?} child = {:?}", token_account.owner, child);
        if token_account.owner != child {
            return Err(error!(MyError::WrongOwner));
        }

        let validation: &mut Account<models::Validation> = &mut ctx.accounts.validation;

        validation.parent = *parent.key;
        validation.child = child;
        let clock: Clock = Clock::get().unwrap();
        validation.timestamp = clock.unix_timestamp;
        Ok(())
    }
}

pub fn name_seed(name: &str) -> &[u8] {
    let b = name.as_bytes();
    if b.len() > 32 {
        &b[0..32]
    } else {
        b
    }
}

#[derive(Accounts)]
// Atencao isso eh posicional
#[instruction(token: Pubkey, min_balance: u64, dao_slug: String)]
pub struct InitDAO<'info> {
    #[account(init, payer = founder, space = models::Zendao::space(&dao_slug),
        seeds = [b"dao".as_ref(), 
            name_seed(&dao_slug).as_ref(),
        ], bump)]
    pub zendao: Account<'info, models::Zendao>,

    #[account(init, payer = founder, space = models::Validation::LEN,
        seeds = [b"child".as_ref(), founder.key.as_ref(), zendao.key().as_ref()], bump)]
    pub validation: Account<'info, models::Validation>,

    #[account(mut)]
    pub founder: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateDAO<'info> {
    #[account(mut)]
    pub zendao: Account<'info, models::Zendao>,

    #[account(mut)]
    pub founder: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseDAO<'info> {
    #[account(mut, close = signer)]
    pub zendao: Account<'info, models::Zendao>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(child: Pubkey)]
pub struct ValidateHuman<'info> {
    #[account(init, payer = parent, space = models::Validation::LEN,
        seeds = [b"child".as_ref(), child.as_ref(), zendao.key().as_ref()], bump)]
    pub validation: Account<'info, models::Validation>,

    #[account()]
    pub token_account: Account<'info, TokenAccount>,

    #[account()]
    pub parent_validation: Account<'info, models::Validation>,

    #[account()]
    pub zendao: Account<'info, models::Zendao>,

    #[account(mut)]
    pub parent: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct ValidateTelegramUser<'info> {
    #[account(init, payer = signer, space = models::TelegramUser::LEN,
        seeds = [b"telegram_user".as_ref(), zendao.key().as_ref(), &id.to_le_bytes()], bump)]
    pub telegram_user: Account<'info, models::TelegramUser>,

    #[account()]
    pub zendao: Account<'info, models::Zendao>,

    #[account()]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferInstruction<'info> {
    /// CHECK: xxx
    pub program_signer: AccountInfo<'info>,

    /// CHECK: xxx
    #[account(signer)] //authority should sign this txn
    pub authority: AccountInfo<'info>,

    pub mint: Account<'info, Mint>,

    /// CHECK: xxx
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    /// CHECK: xxx
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    /// CHECK: xxx
    // We already know its address and that it's executable
    #[account(executable, constraint = token_program.key == &token::ID)]
    pub token_program: AccountInfo<'info>,

    /// CHECK: xxx
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitWallet<'info> {

    #[account(
        init,
        payer = user_sending,
        seeds=[
            b"wallet".as_ref(),
            mint.key().as_ref()
        ],
        bump,
        token::mint=mint,
        token::authority=user_sending,
    )]
    escrow_wallet: Account<'info, TokenAccount>,

    // Users and accounts in the system
    #[account(mut)]
    user_sending: Signer<'info>, // Alice
    mint: Account<'info, Mint>,  // USDC

    /// CHECK: We already know its address and that it's executable
    #[account(executable, constraint = token_program.key == &token::ID)]
    pub token_program: AccountInfo<'info>,

    /// CHECK: xxx
    pub system_program: AccountInfo<'info>,

    rent: Sysvar<'info, Rent>,
}
