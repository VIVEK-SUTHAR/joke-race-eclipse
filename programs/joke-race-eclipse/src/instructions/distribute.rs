use crate::error::ErrorCode;
use crate::state::Vault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Distribute<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub vault: Account<'info, Vault>,
    pub authority: Signer<'info>,
    #[account(mut)]
    ///CHECK: SOl transfer
    pub recipient: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_distribute(ctx: Context<Distribute>, amount: u64) -> Result<()> {
    require!(
        ctx.accounts.vault.authority == *ctx.accounts.authority.key,
        ErrorCode::Unauthorized
    );

    let vault_account_info = ctx.accounts.vault.to_account_info();
    let recipient_account_info = ctx.accounts.recipient.to_account_info();
    **vault_account_info.try_borrow_mut_lamports()? -= amount;
    **recipient_account_info.try_borrow_mut_lamports()? += amount;
    Ok(())
}
