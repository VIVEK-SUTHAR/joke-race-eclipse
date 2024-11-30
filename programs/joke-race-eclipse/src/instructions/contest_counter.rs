use crate::constants::CONTEST_COUNTER_SEED_PREFIX;
use crate::state::ContestCounter;
use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct InitContestCounter<'info> {
    #[account(
        init,
        payer = owner,
        space = ContestCounter::SPACE,
        seeds = [CONTEST_COUNTER_SEED_PREFIX],
        bump
    )]
    pub contest_counter: Account<'info, ContestCounter>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handle_contest_counter(ctx: Context<InitContestCounter>) -> Result<()> {
    ctx.accounts.contest_counter.count = 0;
    Ok(())
}
