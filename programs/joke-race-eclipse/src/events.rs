use anchor_lang::prelude::*;
#[event]
pub struct ContestCreated {
    pub contest_id: u64,
    pub created_by: Pubkey,
    pub metadata_uri: String,
    pub created_at: i64,
    pub start_time: i64,
    pub end_time: i64,
}
#[event]
pub struct VoteCasted {
    pub voted_by: Pubkey,
    pub contest_id: u64,
    pub casted_at: i64,
    pub contestant_id: u64,
}
