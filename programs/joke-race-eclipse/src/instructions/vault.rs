use crate::constants::VAULT_SEED_PREFIX;
use crate::Vault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        seeds=[VAULT_SEED_PREFIX],
        space = Vault::SPACE,
        bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.vault.authority = ctx.accounts.owner.key();
    Ok(())
}
