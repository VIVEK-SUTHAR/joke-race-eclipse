use anchor_lang::prelude::*;
declare_id!("HUTpgdZUCAZpMazn7zo2TnhTF9hjNHK9rDf7zYojFhRQ");

#[program]
pub mod joke_race_eclipse {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault.authority = ctx.accounts.owner.key();
        Ok(())
    }

    pub fn init_contest_counter(ctx: Context<InitContestCounter>) -> Result<()> {
        ctx.accounts.contest_counter.count = 0;
        Ok(())
    }

    pub fn create_contest(
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
        contest.start_time = clock.unix_timestamp;
        contest.end_time = end_time;
        contest_counter.count += 1;

        //Emit Event when The New Contest is created
        emit!(ContestCreated {
            contest_id: count,
            metadata_uri: metadata_uri,
            created_by: *ctx.accounts.author.key,
            created_at: Clock::get()?.unix_timestamp
        });

        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, amount: u64, contestant_id: u64) -> Result<()> {
        let contest = &mut ctx.accounts.contest;
        let voter_record = &mut ctx.accounts.voter_record;

        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;

        if current_time > contest.end_time {
            return err!(ErrorCode::VotingEnded);
        }

        if voter_record.has_voted {
            return err!(ErrorCode::AlreadyVoted);
        }

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
        voter_record.has_voted = true;

        contest.upvotes += 1;

        //Move Sol from User to vault
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
            contest_id: contest.id
        });

        Ok(())
    }

    pub fn distribute(ctx: Context<Distribute>, amount: u64) -> Result<()> {
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
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32,
    )]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitContestCounter<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 8,
        seeds = [b"contest_counter"],
        bump
    )]
    pub contest_counter: Account<'info, ContestCounter>,

    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateContest<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + 256 + 8 + 8 + 32,
        seeds = [b"contest", author.key().as_ref(), &contest_counter.count.to_le_bytes()],
        bump
    )]
    pub contest: Account<'info, Contest>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(
        mut,
        seeds = [b"contest_counter"],
        bump
    )]
    pub contest_counter: Account<'info, ContestCounter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64, contestant_id: u64)]
pub struct Vote<'info> {
    #[account(
        mut,
        seeds = [b"contest", contest.author.as_ref() , &contest.id.to_le_bytes()],
        bump
    )]
    pub contest: Account<'info, Contest>,
    #[account(
    init_if_needed,
    payer = voter,
    space = 8 + 32 + 32 + 8 + 1,
    seeds = [b"vote_account",voter.key().as_ref() ,&contest.id.to_le_bytes() ,&contestant_id.to_le_bytes()],
    bump
)]
    pub voter_record: Account<'info, VoterContestRecord>,
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

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

#[account]
pub struct Vault {
    pub authority: Pubkey,
}

#[account]
pub struct Contest {
    pub id: u64,
    pub metadata_uri: String,
    pub upvotes: u64,
    pub author: Pubkey,
    pub start_time: i64,
    pub end_time: i64,
}

#[account]
pub struct ContestCounter {
    pub count: u64,
}

#[account]
pub struct VoterContestRecord {
    pub contest_id: u64,
    pub contestant_id: [u8; 32],
    pub voter: Pubkey,
    pub has_voted: bool,
}
#[error_code]
pub enum ErrorCode {
    AlreadyVoted,
    VotingEnded,
    Unauthorized,
}

#[event]
pub struct ContestCreated {
    contest_id: u64,
    created_by: Pubkey,
    metadata_uri: String,
    created_at: i64,
}
#[event]
pub struct VoteCasted {
    voted_by: Pubkey,
    contest_id: u64,
    casted_at: i64,
}
