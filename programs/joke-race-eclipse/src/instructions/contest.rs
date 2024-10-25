use anchor_lang::prelude::*;

use crate::constants::{CONTEST_COUNTER_SEED_PREFIX, CONTEST_SEED_PREFIX};
use crate::events::ContestCreated;
use crate::state::Contest;
use crate::state::ContestCounter;

#[derive(Accounts)]
pub struct CreateContest<'info> {
    #[account(
        init,
        payer = author,
        space = Contest::SPACE,
        seeds = [CONTEST_SEED_PREFIX, author.key().as_ref(), &contest_counter.count.to_le_bytes()],
        bump
    )]
    pub contest: Account<'info, Contest>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTEST_COUNTER_SEED_PREFIX],
        bump
    )]
    pub contest_counter: Account<'info, ContestCounter>,
    pub system_program: Program<'info, System>,
}

pub fn handle_create_contest(
    ctx: Context<CreateContest>,
    metadata_uri: String,
    end_time: i64,
) -> Result<()> {
    let contest_counter = &mut ctx.accounts.contest_counter;
    let count = contest_counter.count;

    let contest = &mut ctx.accounts.contest;

    contest.id = count;
    contest.metadata_uri = metadata_uri.clone();
    contest.upvotes = 0;
    contest.author = ctx.accounts.author.key();

    let clock = Clock::get()?;
    contest_counter.count += 1;

    let start_time = clock.unix_timestamp;
    contest.start_time = start_time;
    contest.end_time = end_time;

    //Emit Event when The New Contest is created
    emit!(ContestCreated {
        contest_id: count,
        metadata_uri: metadata_uri,
        created_by: *ctx.accounts.author.key,
        created_at: Clock::get()?.unix_timestamp,
        start_time: start_time,
        end_time: end_time
    });

    Ok(())
}
