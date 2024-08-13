use anchor_lang::prelude::*;
mod poetry;

declare_id!("6cesC5Z7g527BeuDpDKyzP7rjgn17HfKEshspHXYFaUL");


#[program]
pub mod crypto_poetry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let poetry_account = &mut ctx.accounts.poetry_account;
    
        poetry_account.poem = poetry::generate_poem(&poetry_account.key());
        poetry_account.owner = ctx.accounts.user.key();
        poetry_account.locked = false;
        Ok(())
    }

    pub fn generate_poetry(ctx: Context<GeneratePoetry>) -> Result<()> {
        let poetry_account = &mut ctx.accounts.poetry_account;
    
        require!(!poetry_account.locked, ProgramError::AccountLocked);
    
        let poem = poetry::generate_poem(&poetry_account.key());
        poetry_account.poem = poem;
    
        Ok(())
    }

    pub fn close_poetry_account(_ctx: Context<ClosePoetryAccount>) -> Result<()> {
        // The account is closed automatically by the anchor attribute
        Ok(())
    }

    pub fn lock_poetry_account(ctx: Context<LockPoetryAccount>) -> Result<()> {
        let poetry_account = &mut ctx.accounts.poetry_account;
        poetry_account.locked = true;
        Ok(())
    }

}

#[error_code]
pub enum ProgramError {
    #[msg("This account is locked and cannot generate new poems")]
    AccountLocked,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 356 + 32 + 8)]
    pub poetry_account: Account<'info, PoetryAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockPoetryAccount<'info> {
    #[account(mut, has_one = owner)]
    pub poetry_account: Account<'info, PoetryAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct GeneratePoetry<'info> {
    #[account(mut, has_one = owner)]
    pub poetry_account: Account<'info, PoetryAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClosePoetryAccount<'info> {
    #[account(mut, close = owner, has_one = owner)]
    pub poetry_account: Account<'info, PoetryAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct PoetryAccount {
    pub poem: String,
    pub owner: Pubkey,
    pub locked: bool,
}
