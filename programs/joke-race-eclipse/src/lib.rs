use anchor_lang::prelude::*;

mod constants;
mod error;
mod events;
pub mod instructions;
pub mod state;
use instructions::*;
use state::*;

declare_id!("CGfVb6h6wHnAxLfMw3EnssxBEPQgdyNSQzXtWVYkx7Vn");

#[program]
pub mod joke_race_eclipse {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        handle_initialize(ctx)
    }

    pub fn init_contest_counter(ctx: Context<InitContestCounter>) -> Result<()> {
        handle_contest_counter(ctx)
    }

    pub fn create_contest(
        ctx: Context<CreateContest>,
        metadata_uri: String,
        start_time: i64,
        end_time: i64,
    ) -> Result<()> {
        handle_create_contest(ctx, metadata_uri, start_time, end_time)
    }

    pub fn vote(ctx: Context<Vote>, amount: u64, contestant_id: u64) -> Result<()> {
        handle_vote(ctx, amount, contestant_id)
    }

    pub fn distribute(ctx: Context<Distribute>, amount: u64) -> Result<()> {
        handle_distribute(ctx, amount)
    }
}
