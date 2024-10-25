use crate::constants::{CONTEST_SEED_PREFIX, VOTE_ACCOUNT_SEED_PREFIX};
use crate::error::ErrorCode;
use crate::events::VoteCasted;
use crate::state::{Contest, Vault, VoterContestRecord};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(amount: u64, contestant_id: u64)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [CONTEST_SEED_PREFIX, contest.author.as_ref() , &contest.id.to_le_bytes()],
        bump
    )]
    pub contest: Account<'info, Contest>,
    #[account(
    init_if_needed,
    payer = voter,
    space = VoterContestRecord::SPACE,
    seeds = [VOTE_ACCOUNT_SEED_PREFIX,voter.key().as_ref() ,&contest.id.to_le_bytes() ,&contestant_id.to_le_bytes()],
    bump
)]
    pub voter_record: Account<'info, VoterContestRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

pub fn handle_vote(ctx: Context<Vote>, amount: u64, contestant_id: u64) -> Result<()> {
    let contest = &mut ctx.accounts.contest;
    let voter_record = &mut ctx.accounts.voter_record;

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    if current_time > contest.end_time {
        return err!(ErrorCode::VotingEnded);
    }

    //A User can Vote on Same project Multiple Times
    // if voter_record.has_voted {
    //     return err!(ErrorCode::AlreadyVoted);
    // }
    voter_record.total_votes += 1;

    // track of who voted by PubKey and has_voted
    voter_record.voter = *ctx.accounts.voter.key;

    // First,we Convert the u64 contestant ID into a byte array.
    //`to_le_bytes()` function converts the u64 into an array of 8 bytes (since the size of a u64 is 8 bytes).
    let contestant_id_bytes = contestant_id.to_le_bytes();

    //Then we, create a new array of 32 bytes, as we need a 32-byte sized array for padding.
    //initialize this array with all zeros (using `0u8`), and it will have a fixed length of 32.
    let mut padded_contestant_id = [0u8; 32];

    // Copy the original 8-byte `contestant_id_bytes` into the first 8 bytes of  padded 32-byte array.
    // The rest of the array remains filled with zeros, allowing us to meet the 32-byte size requirement.
    padded_contestant_id[..8].copy_from_slice(&contestant_id_bytes);

    voter_record
        .contestant_id
        .copy_from_slice(&padded_contestant_id);

    voter_record.contest_id = contest.id;

    contest.upvotes += 1;

    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.voter.key(),
        &ctx.accounts.vault.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.voter.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    emit!(VoteCasted {
        voted_by: *ctx.accounts.voter.key,
        casted_at: Clock::get()?.unix_timestamp,
        contest_id: contest.id,
        contestant_id: contestant_id
    });

    Ok(())
}
